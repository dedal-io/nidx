use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct NidInfo {
    country: String,
    birthday: String,
    sex: String,
    is_national: bool,
    year: u16,
    month: u8,
    day: u8,
}

#[wasm_bindgen]
impl NidInfo {
    #[wasm_bindgen(getter)]
    pub fn country(&self) -> String {
        self.country.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn birthday(&self) -> String {
        self.birthday.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn sex(&self) -> String {
        self.sex.clone()
    }

    #[wasm_bindgen(getter, js_name = "isNational")]
    pub fn is_national(&self) -> bool {
        self.is_national
    }

    #[wasm_bindgen(getter)]
    pub fn year(&self) -> u16 {
        self.year
    }

    #[wasm_bindgen(getter)]
    pub fn month(&self) -> u8 {
        self.month
    }

    #[wasm_bindgen(getter)]
    pub fn day(&self) -> u8 {
        self.day
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        let obj = js_sys::Object::new();
        let set = |k: &str, v: &JsValue| -> Result<(), JsError> {
            js_sys::Reflect::set(&obj, &k.into(), v)
                .map(|_| ())
                .map_err(|e| JsError::new(&format!("failed to set property '{k}': {e:?}")))
        };
        set("country", &self.country.as_str().into())?;
        set("birthday", &self.birthday.as_str().into())?;
        set("sex", &self.sex.as_str().into())?;
        set("isNational", &self.is_national.into())?;
        set("year", &self.year.into())?;
        set("month", &self.month.into())?;
        set("day", &self.day.into())?;
        Ok(obj.into())
    }
}

fn albania_to_js_error(e: nidx::albania::NidError) -> JsError {
    let code = match &e {
        nidx::albania::NidError::Format(_) => "FORMAT",
        nidx::albania::NidError::Checksum => "CHECKSUM",
        nidx::albania::NidError::InvalidDate(_) => "INVALID_DATE",
        _ => "UNKNOWN",
    };
    JsError::new(&format!("[{code}] {e}"))
}

/// Decode an Albanian National ID.
#[wasm_bindgen(js_name = "albaniaDecode")]
pub fn albania_decode(nid: &str) -> Result<NidInfo, JsError> {
    let info = nidx::albania::decode(nid).map_err(albania_to_js_error)?;
    Ok(NidInfo {
        country: "albania".to_string(),
        birthday: info.birthday.to_string(),
        sex: info.sex.to_string(),
        is_national: info.is_national,
        year: info.birthday.year,
        month: info.birthday.month,
        day: info.birthday.day,
    })
}

/// Check whether an Albanian National ID string is valid.
#[wasm_bindgen(js_name = "albaniaIsValid")]
pub fn albania_is_valid(nid: &str) -> bool {
    nidx::albania::is_valid(nid)
}
