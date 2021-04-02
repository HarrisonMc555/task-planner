use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::ops::Deref;

// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
// https://github.com/SergioBenitez/Rocket/issues/602#issuecomment-380497269
#[derive(Debug)]
pub struct NaiveDateForm(pub NaiveDate);
#[derive(Debug)]
pub struct NaiveTimeForm(pub NaiveTime);
#[derive(Debug)]
pub struct NaiveDateTimeForm(pub NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
            return Ok(NaiveDateForm(date));
        }
        Err(form_value)
    }
}

impl<'v> FromFormValue<'v> for NaiveTimeForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M:%S%.3f") {
            // if time.nanosecond() >= 1_000_000_000 {
            //     return Err(form_value);
            // }
            return Ok(NaiveTimeForm(time));
        }
        if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M") {
            return Ok(NaiveTimeForm(time));
        }
        Err(form_value)
    }
}

impl<'v> FromFormValue<'v> for NaiveDateTimeForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTimeForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if decoded.len() < "0000-00-00T00:00".len() {
            return Err(form_value);
        }
        let date = NaiveDateForm::from_form_value(RawStr::from_str(&decoded[.."0000-00-00".len()]))
            .map_err(|_| form_value)?;
        let time =
            NaiveTimeForm::from_form_value(RawStr::from_str(&decoded["0000-00-00T".len()..]))
                .map_err(|_| form_value)?;
        Ok(NaiveDateTimeForm(NaiveDateTime::new(*date, *time)))
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}

impl Deref for NaiveTimeForm {
    type Target = NaiveTime;
    fn deref(&self) -> &NaiveTime {
        &self.0
    }
}

impl Deref for NaiveDateTimeForm {
    type Target = NaiveDateTime;
    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }
}
