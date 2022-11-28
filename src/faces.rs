use embedded_graphics::image::ImageRaw;
use embedded_graphics::pixelcolor::BinaryColor;
use esp_idf_sys::esp_random;
use std::collections::HashMap;

pub struct Faces {
    data: HashMap<&'static str, Vec<u8>>,
}

impl Faces {
    pub fn new() -> Self {
        let mut data: HashMap<&str, Vec<u8>> = HashMap::new();

        data.insert("angry", include_bytes!("../data/img/angry.raw").to_vec());
        data.insert("awake", include_bytes!("../data/img/awake.raw").to_vec());
        data.insert("bored", include_bytes!("../data/img/bored.raw").to_vec());
        data.insert("broken", include_bytes!("../data/img/broken.raw").to_vec());
        data.insert("cool", include_bytes!("../data/img/cool.raw").to_vec());
        data.insert("debug", include_bytes!("../data/img/debug.raw").to_vec());
        data.insert(
            "demotivated",
            include_bytes!("../data/img/demotivated.raw").to_vec(),
        );
        data.insert(
            "excited",
            include_bytes!("../data/img/excited.raw").to_vec(),
        );
        data.insert("friend", include_bytes!("../data/img/friend.raw").to_vec());
        data.insert(
            "grateful",
            include_bytes!("../data/img/grateful.raw").to_vec(),
        );
        data.insert("happy", include_bytes!("../data/img/happy.raw").to_vec());
        data.insert(
            "intense",
            include_bytes!("../data/img/intense.raw").to_vec(),
        );
        data.insert("lonely", include_bytes!("../data/img/lonely.raw").to_vec());
        data.insert("look_l", include_bytes!("../data/img/look_l.raw").to_vec());
        data.insert(
            "look_l_happy",
            include_bytes!("../data/img/look_l_happy.raw").to_vec(),
        );
        data.insert("look_r", include_bytes!("../data/img/look_r.raw").to_vec());
        data.insert(
            "look_r_happy",
            include_bytes!("../data/img/look_r_happy.raw").to_vec(),
        );
        data.insert(
            "motivated",
            include_bytes!("../data/img/motivated.raw").to_vec(),
        );
        data.insert("sad", include_bytes!("../data/img/sad.raw").to_vec());

        Self { data }
    }

    pub fn random(&mut self) -> Result<ImageRaw<BinaryColor>, String> {
        let mut random_number = unsafe { esp_random() };

        let arr_length = self.data.len() as u32;
        if random_number > arr_length {
            random_number = random_number % arr_length;
        }

        let mut i = 0;

        for (key, value) in &self.data {
            if i == random_number {
                let img = ImageRaw::<BinaryColor>::new(value.as_slice(), 200);

                return Ok(img);
            }

            i = i + 1;
        }

        return Err("not found".to_string());
    }
}
