use esp_idf_sys::*;
use ieee80211::*;
use std::collections::HashMap;

struct NetworkInfo {
    ssid: &'static str,
    mac: MacAddress,
}

pub struct WiFi {
    channel: u8,
    known_networks: HashMap<&'static str, NetworkInfo>,
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
        Self {
            channel: ch,
            known_networks: HashMap::new(),
        }
    }

    pub fn set_channel(&mut self, ch: u8) {
        println!("Changing channel to {}", ch);
        unsafe {
            esp!(esp_wifi_set_channel(
                ch,
                wifi_second_chan_t_WIFI_SECOND_CHAN_NONE
            ))
            .unwrap()
        };
        self.channel = ch;
    }

    pub fn next_channel(&mut self) {
        if self.channel >= 13 {
            self.channel = 1;
        }

        self.set_channel(self.channel);

        self.channel = self.channel + 1;
    }
}

pub extern "C" fn pkg_callback(buf: *mut c_types::c_void, type_: wifi_promiscuous_pkt_type_t) {
    let mut pkg_type = "";

    match type_ {
        wifi_promiscuous_pkt_type_t_WIFI_PKT_MGMT => pkg_type = "MGMT",
        wifi_promiscuous_pkt_type_t_WIFI_PKT_CTRL => pkg_type = "CTRL",
        wifi_promiscuous_pkt_type_t_WIFI_PKT_MISC => pkg_type = "MISC",
        wifi_promiscuous_pkt_type_t_WIFI_PKT_DATA => pkg_type = "DATA",
        _ => unreachable!(),
    }

    let pkt_data: &mut wifi_promiscuous_pkt_t =
        unsafe { &mut *(buf as *mut wifi_promiscuous_pkt_t) };

    if pkt_data.rx_ctrl.rx_state() != 0 {
        println!("Broken pkg, ignoring");

        return;
    }

    let mut pkt_length = pkt_data.rx_ctrl.sig_len() as usize;
    pkt_length = pkt_length - 4; // fix for https://github.com/espressif/esp-idf/issues/886

    let data = unsafe { pkt_data.payload.as_slice(pkt_length) };
    let frame = Frame::new(data);

    println!(
        "Got a pkg of type {} with length of {}bytes",
        pkg_type,
        pkt_data.rx_ctrl.sig_len()
    );

    let layer = frame.next_layer().unwrap();

    if let FrameLayer::Management(ref management_frame) = layer {
        let management_frame_layer = management_frame.next_layer().unwrap();
        match management_frame_layer {
            ManagementFrameLayer::Beacon(ref BeaconFrame) => {
                if BeaconFrame.version().into_u8() != 0 {
                    println!("Frame version != 0, ignoring");

                    return;
                }
                match BeaconFrame.ssid() {
                    Some(v) => println!(
                        "Beacon for SSID: len {}, {}, {}",
                        v.len(),
                        String::from_utf8(v).unwrap(),
                        BeaconFrame.addr2().to_hex_string()
                    ),
                    None => println!("Beacon without SSID"),
                }
            }
            ManagementFrameLayer::ProbeRequest(ref ProbeRequestFrame) => println!(
                "ProbeRequest, SSID: {}",
                String::from_utf8(ProbeRequestFrame.ssid().unwrap()).unwrap()
            ),
            ManagementFrameLayer::ProbeResponse(ProbeResponseFrame) => println!("ProbeResponse"),
            ManagementFrameLayer::Authentication(AuthenticationFrame) => println!("Authentication"),
            ManagementFrameLayer::Deauthentication(DeauthenticationFrame) => {
                println!("Deauthentication")
            }
            ManagementFrameLayer::Disassociate(DisassociateFrame) => println!("Disassociate"),
            ManagementFrameLayer::AssociationRequest(AssociationRequestFrame) => {
                println!("AssociationRequest")
            }
            ManagementFrameLayer::AssociationResponse(AssociationResponseFrame) => {
                println!("AssociationResponse")
            }
        }
    }
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
