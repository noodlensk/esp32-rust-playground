use esp_idf_sys::{
    c_types, esp, esp_event_send_internal, esp_netif_init, esp_wifi_init, esp_wifi_set_channel,
    esp_wifi_set_mode, esp_wifi_set_promiscuous, esp_wifi_set_promiscuous_filter,
    esp_wifi_set_promiscuous_rx_cb, esp_wifi_set_storage, esp_wifi_start,
    g_wifi_default_wpa_crypto_funcs, g_wifi_feature_caps, g_wifi_osi_funcs, nvs_flash_init,
    wifi_init_config_t, wifi_mode_t_WIFI_MODE_NULL, wifi_promiscuous_filter_t,
    wifi_promiscuous_pkt_t, wifi_promiscuous_pkt_type_t, wifi_promiscuous_pkt_type_t_WIFI_PKT_CTRL,
    wifi_promiscuous_pkt_type_t_WIFI_PKT_DATA, wifi_promiscuous_pkt_type_t_WIFI_PKT_MGMT,
    wifi_promiscuous_pkt_type_t_WIFI_PKT_MISC, wifi_second_chan_t_WIFI_SECOND_CHAN_NONE,
    wifi_storage_t_WIFI_STORAGE_RAM, CONFIG_ESP32_WIFI_DYNAMIC_RX_BUFFER_NUM,
    CONFIG_ESP32_WIFI_STATIC_RX_BUFFER_NUM, CONFIG_ESP32_WIFI_TX_BUFFER_TYPE,
    WIFI_AMPDU_RX_ENABLED, WIFI_AMPDU_TX_ENABLED, WIFI_AMSDU_TX_ENABLED, WIFI_CACHE_TX_BUFFER_NUM,
    WIFI_CSI_ENABLED, WIFI_DEFAULT_RX_BA_WIN, WIFI_DYNAMIC_TX_BUFFER_NUM, WIFI_INIT_CONFIG_MAGIC,
    WIFI_MGMT_SBUF_NUM, WIFI_NANO_FORMAT_ENABLED, WIFI_NVS_ENABLED, WIFI_PROMIS_FILTER_MASK_DATA,
    WIFI_PROMIS_FILTER_MASK_MGMT, WIFI_SOFTAP_BEACON_MAX_LEN, WIFI_STATIC_TX_BUFFER_NUM,
    WIFI_TASK_CORE_ID,
};
use ieee80211::{
    Frame, FrameLayer, FrameTrait, MacAddress, ManagementFrameLayer, ManagementFrameTrait,
    TaggedParametersTrait,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

pub struct NetworkInfo {
    pub ssid: String,
    pub mac: MacAddress,
}

pub struct WiFi {
    channel: u8,
}

lazy_static! {
    static ref KNOWN_NETWORKS: Mutex<HashMap<String, NetworkInfo>> = Mutex::new(HashMap::new());
}

impl WiFi {
    pub fn new() -> Self {
        let ch: u8 = 1;

        unsafe {
            esp!(nvs_flash_init()).unwrap();
            esp!(esp_netif_init()).unwrap();
            let cfg: wifi_init_config_t = default_init_config();
            esp!(esp_wifi_init(&cfg)).unwrap();

            esp!(esp_wifi_set_storage(wifi_storage_t_WIFI_STORAGE_RAM)).unwrap();
            esp!(esp_wifi_set_mode(wifi_mode_t_WIFI_MODE_NULL)).unwrap();
            esp!(esp_wifi_start()).unwrap();

            let filter: wifi_promiscuous_filter_t = wifi_promiscuous_filter_t {
                filter_mask: WIFI_PROMIS_FILTER_MASK_MGMT | WIFI_PROMIS_FILTER_MASK_DATA,
            };
            esp!(esp_wifi_set_promiscuous_filter(&filter)).unwrap();

            esp!(esp_wifi_set_promiscuous_rx_cb(Some(pkg_callback))).unwrap();
            esp!(esp_wifi_set_promiscuous(true)).unwrap();
            esp!(esp_wifi_set_channel(
                ch,
                wifi_second_chan_t_WIFI_SECOND_CHAN_NONE
            ))
            .unwrap();
        }
        Self { channel: ch }
    }

    pub fn set_channel(&mut self, ch: u8) {
        println!("Changing channel to {}", ch);
        unsafe {
            esp!(esp_wifi_set_channel(
                ch,
                wifi_second_chan_t_WIFI_SECOND_CHAN_NONE
            ))
            .unwrap();
        };
        self.channel = ch;
    }

    pub fn next_channel(&mut self) {
        if self.channel >= 13 {
            self.channel = 1;
        }

        self.set_channel(self.channel);

        self.channel += 1;
    }

    pub fn known_networks() -> MutexGuard<'static, HashMap<String, NetworkInfo>> {
        KNOWN_NETWORKS.lock().unwrap()
    }
}

#[allow(non_upper_case_globals)]
pub extern "C" fn pkg_callback(buf: *mut c_types::c_void, type_: wifi_promiscuous_pkt_type_t) {
    let _pkg_type = match type_ {
        wifi_promiscuous_pkt_type_t_WIFI_PKT_MGMT => "MGMT",
        wifi_promiscuous_pkt_type_t_WIFI_PKT_CTRL => "CTRL",
        wifi_promiscuous_pkt_type_t_WIFI_PKT_MISC => "MISC",
        wifi_promiscuous_pkt_type_t_WIFI_PKT_DATA => "DATA",
        _ => unreachable!(),
    };

    let pkt_data: &mut wifi_promiscuous_pkt_t =
        unsafe { &mut *buf.cast::<wifi_promiscuous_pkt_t>() };

    if pkt_data.rx_ctrl.rx_state() != 0 {
        println!("Broken pkg, ignoring");

        return;
    }

    let mut pkt_length = pkt_data.rx_ctrl.sig_len() as usize;
    pkt_length -= 4; // fix for https://github.com/espressif/esp-idf/issues/886

    let data = unsafe { pkt_data.payload.as_slice(pkt_length) };
    let frame = Frame::new(data);

    if let Err(error) = callback(&frame) {
        println!("Got error: {}", error);
    }
}

fn callback(frame: &Frame) -> Result<(), String> {
    let layer = match frame.next_layer() {
        Some(layer) => layer,
        None => return Err(String::from("get next layer from root")),
    };

    if let FrameLayer::Management(ref management_frame) = layer {
        let management_frame_layer = match management_frame.next_layer() {
            Some(layer) => layer,
            None => return Err(String::from("get next layer from management")),
        };

        match management_frame_layer {
            ManagementFrameLayer::Beacon(ref beacon_frame) => {
                if beacon_frame.version().into_u8() != 0 {
                    return Err(String::from("Frame version != 0, ignoring"));
                }

                match beacon_frame.ssid() {
                    Some(v) => {
                        let mac: String = beacon_frame.to_owned().addr2().to_hex_string();
                        let ssid = match String::from_utf8(v) {
                            Ok(string) => string,
                            Err(e) => return Err(e.to_string()),
                        };

                        let mac_parsed = match MacAddress::parse_str(mac.as_str()) {
                            Ok(mac) => mac,
                            Err(e) => return Err(e.to_string()),
                        };

                        let key = mac_parsed.clone().to_hex_string();

                        KNOWN_NETWORKS
                            .lock()
                            .unwrap()
                            .entry(key)
                            .or_insert(NetworkInfo {
                                ssid,
                                mac: mac_parsed,
                            });
                    }
                    None => println!("Beacon without SSID"),
                }
            }
            ManagementFrameLayer::ProbeRequest(_probe_request_frame) => println!("ProbeRequest"),
            ManagementFrameLayer::ProbeResponse(_probe_response_frame) => println!("ProbeResponse"),
            ManagementFrameLayer::Authentication(_authentication_frame) => {
                println!("Authentication");
            }
            ManagementFrameLayer::Deauthentication(_deauthentication_frame) => {
                println!("Deauthentication");
            }
            ManagementFrameLayer::Disassociate(_disassociate_frame) => println!("Disassociate"),
            ManagementFrameLayer::AssociationRequest(_association_request_frame) => {
                println!("AssociationRequest");
            }
            ManagementFrameLayer::AssociationResponse(_association_response_frame) => {
                println!("AssociationResponse");
            }
        }
    }

    Ok(())
}

unsafe fn default_init_config() -> wifi_init_config_t {
    wifi_init_config_t {
        event_handler: Some(esp_event_send_internal),
        osi_funcs: &mut g_wifi_osi_funcs,
        wpa_crypto_funcs: g_wifi_default_wpa_crypto_funcs,
        static_rx_buf_num: CONFIG_ESP32_WIFI_STATIC_RX_BUFFER_NUM as i32,
        dynamic_rx_buf_num: CONFIG_ESP32_WIFI_DYNAMIC_RX_BUFFER_NUM as i32,
        tx_buf_type: CONFIG_ESP32_WIFI_TX_BUFFER_TYPE as i32,
        static_tx_buf_num: WIFI_STATIC_TX_BUFFER_NUM as i32,
        dynamic_tx_buf_num: WIFI_DYNAMIC_TX_BUFFER_NUM as i32,
        cache_tx_buf_num: WIFI_CACHE_TX_BUFFER_NUM as i32,
        csi_enable: WIFI_CSI_ENABLED as i32,
        ampdu_rx_enable: WIFI_AMPDU_RX_ENABLED as i32,
        ampdu_tx_enable: WIFI_AMPDU_TX_ENABLED as i32,
        amsdu_tx_enable: WIFI_AMSDU_TX_ENABLED as i32,
        nvs_enable: WIFI_NVS_ENABLED as i32,
        nano_enable: WIFI_NANO_FORMAT_ENABLED as i32,
        rx_ba_win: WIFI_DEFAULT_RX_BA_WIN as i32,
        wifi_task_core_id: WIFI_TASK_CORE_ID as i32,
        beacon_max_len: WIFI_SOFTAP_BEACON_MAX_LEN as i32,
        mgmt_sbuf_num: WIFI_MGMT_SBUF_NUM as i32,
        feature_caps: g_wifi_feature_caps as u64,
        sta_disconnected_pm: false,
        magic: WIFI_INIT_CONFIG_MAGIC as i32,
    }
}
