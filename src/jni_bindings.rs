use crate::{parse_content, parse_to_json as internal_parse_to_json};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jboolean, jobject, jstring};

/// JNI binding for parsing to JSON string
#[unsafe(no_mangle)]
pub extern "system" fn Java_ovh_sad_commonmeta_CommonMetaParser_parseToJson(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let result_json = internal_parse_to_json(&input_str);

    match env.new_string(result_json) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// JNI binding for parsing to HashMap
#[unsafe(no_mangle)]
pub extern "system" fn Java_ovh_sad_commonmeta_CommonMetaParser_parse(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jobject {
    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let result = parse_content(&input_str);

    if !result.success {
        return std::ptr::null_mut();
    }

    // Create a Java HashMap
    let hashmap_class = match env.find_class("java/util/HashMap") {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };

    let hashmap = match env.new_object(&hashmap_class, "()V", &[]) {
        Ok(obj) => obj,
        Err(_) => return std::ptr::null_mut(),
    };

    // Add all pairs to the HashMap
    for (key, value) in result.pairs {
        let jkey_obj = match env.new_string(key) {
            Ok(s) => JObject::from(s),
            Err(_) => continue,
        };

        let jvalue_obj = match env.new_string(value) {
            Ok(s) => JObject::from(s),
            Err(_) => continue,
        };

        let jkey = JValue::Object(&jkey_obj);
        let jvalue = JValue::Object(&jvalue_obj);

        let _ = env.call_method(
            &hashmap,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[jkey, jvalue],
        );
    }

    hashmap.into_raw()
}

/// JNI binding for validation
#[unsafe(no_mangle)]
pub extern "system" fn Java_ovh_sad_commonmeta_CommonMetaParser_validate(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jboolean {
    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return 0u8,
    };

    let result = parse_content(&input_str);
    if result.success { 1u8 } else { 0u8 }
}

/// JNI binding for getting error message
#[unsafe(no_mangle)]
pub extern "system" fn Java_ovh_sad_commonmeta_CommonMetaParser_getError(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let result = parse_content(&input_str);

    if let Some(error) = result.error {
        match env.new_string(error) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}
