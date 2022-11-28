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

        data.insert(
            "angry",
            include_bytes!("../data/img/raw/angry.raw").to_vec(),
        );
        data.insert(
            "awake",
            include_bytes!("../data/img/raw/awake.raw").to_vec(),
        );
        data.insert(
            "bored",
            include_bytes!("../data/img/raw/bored.raw").to_vec(),
        );
        data.insert(
            "broken",
            include_bytes!("../data/img/raw/broken.raw").to_vec(),
        );
        data.insert("cool", include_bytes!("../data/img/raw/cool.raw").to_vec());
        data.insert(
            "debug",
            include_bytes!("../data/img/raw/debug.raw").to_vec(),
        );
        data.insert(
            "demotivated",
            include_bytes!("../data/img/raw/demotivated.raw").to_vec(),
        );
        data.insert(
            "excited",
            include_bytes!("../data/img/raw/excited.raw").to_vec(),
        );
        data.insert(
            "friend",
            include_bytes!("../data/img/raw/friend.raw").to_vec(),
        );
        data.insert(
            "grateful",
            include_bytes!("../data/img/raw/grateful.raw").to_vec(),
        );
        data.insert(
            "happy",
            include_bytes!("../data/img/raw/happy.raw").to_vec(),
        );
        data.insert(
            "intense",
            include_bytes!("../data/img/raw/intense.raw").to_vec(),
        );
        data.insert(
            "lonely",
            include_bytes!("../data/img/raw/lonely.raw").to_vec(),
        );
        data.insert(
            "look_l",
            include_bytes!("../data/img/raw/look_l.raw").to_vec(),
        );
        data.insert(
            "look_l_happy",
            include_bytes!("../data/img/raw/look_l_happy.raw").to_vec(),
        );
        data.insert(
            "look_r",
            include_bytes!("../data/img/raw/look_r.raw").to_vec(),
        );
        data.insert(
            "look_r_happy",
            include_bytes!("../data/img/raw/look_r_happy.raw").to_vec(),
        );
        data.insert(
            "motivated",
            include_bytes!("../data/img/raw/motivated.raw").to_vec(),
        );
        data.insert("sad", include_bytes!("../data/img/raw/sad.raw").to_vec());

        Self { data }
    }

    pub fn random(&mut self) -> Result<ImageRaw<BinaryColor>, String> {
        let mut random_number = unsafe { esp_random() };

        let arr_length = self.data.len() as u32;
        if random_number > arr_length {
            random_number %= arr_length;
        }

        for (i, (_key, value)) in self.data.iter().enumerate() {
            if i as u32 == random_number {
                let img = ImageRaw::<BinaryColor>::new(value.as_slice(), 200);

                return Ok(img);
            }
        }

        Err("not found".to_string())
    }
}
