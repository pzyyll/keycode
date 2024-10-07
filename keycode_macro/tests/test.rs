use keycode_macro::parse_keycode_converter_data;

parse_keycode_converter_data!();

#[test]
fn keymappind_id_from_string() {
    // assert_eq!(KeyModifiers::from_bits(0b0001_0001).unwrap(), KeyModifiers::Control);

    assert_eq!(KeyMappingId::from_str("UsA").unwrap(), KeyMappingId::UsA);
    assert_eq!(
        KeyMappingId::from_str("ShiftLeft").unwrap(),
        KeyMappingId::ShiftLeft
    );
    assert_eq!(
        KeyMappingId::from_str("AltRight").unwrap(),
        KeyMappingId::AltRight
    );
    assert_eq!(
        KeyMappingId::from_str("MetaRight").unwrap(),
        KeyMappingId::MetaRight
    );
    assert_eq!(
        KeyMappingId::from_str("ControlRight").unwrap(),
        KeyMappingId::ControlRight
    );
    assert_eq!(
        KeyMappingId::from_str("ShiftRight").unwrap(),
        KeyMappingId::ShiftRight
    );
    assert_eq!(
        KeyMappingId::from_str("AltLeft").unwrap(),
        KeyMappingId::AltLeft
    );
    assert_eq!(
        KeyMappingId::from_str("MetaLeft").unwrap(),
        KeyMappingId::MetaLeft
    );
    assert_eq!(
        KeyMappingId::from_str("ControlLeft").unwrap(),
        KeyMappingId::ControlLeft
    );
}

#[test]
fn keymapping_code_from_string() {
    assert_eq!(
        KeyMappingCode::from_str("KeyA").unwrap(),
        KeyMappingCode::KeyA
    );
    assert_eq!(
        KeyMappingCode::from_str("KeyB").unwrap(),
        KeyMappingCode::KeyB
    );
    assert_eq!(
        KeyMappingCode::from_str("KeyC").unwrap(),
        KeyMappingCode::KeyC
    );
    assert_eq!(
        KeyMappingCode::from_str("KeyD").unwrap(),
        KeyMappingCode::KeyD
    );
    assert_eq!(
        KeyMappingCode::from_str("KeyE").unwrap(),
        KeyMappingCode::KeyE
    );
    assert_eq!(
        KeyMappingCode::from_str("ShiftLeft").unwrap(),
        KeyMappingCode::ShiftLeft
    );
    assert_eq!(
        KeyMappingCode::from_str("ShiftRight").unwrap(),
        KeyMappingCode::ShiftRight
    );
    assert_eq!(
        KeyMappingCode::from_str("AltLeft").unwrap(),
        KeyMappingCode::AltLeft
    );
    assert_eq!(
        KeyMappingCode::from_str("AltRight").unwrap(),
        KeyMappingCode::AltRight
    );
    assert_eq!(
        KeyMappingCode::from_str("MetaLeft").unwrap(),
        KeyMappingCode::MetaLeft
    );
    assert_eq!(
        KeyMappingCode::from_str("MetaRight").unwrap(),
        KeyMappingCode::MetaRight
    );
    assert_eq!(
        KeyMappingCode::from_str("ControlLeft").unwrap(),
        KeyMappingCode::ControlLeft
    );
    assert_eq!(
        KeyMappingCode::from_str("ControlRight").unwrap(),
        KeyMappingCode::ControlRight
    );
}

#[test]
fn virtual_key_map() {
    assert_eq!(
        VirtualKeyId::Alt.to_key_mapping_id(),
        None
    );
    
    assert_eq!(
        VirtualKeyId::UsA.to_key_mapping_id(),
        Some(KeyMappingId::UsA)
    );

    assert_eq!(
        VirtualKeyId::UsB.to_key_mapping_id(),
        Some(KeyMappingId::UsB)
    );

    VirtualKeyId::Control.to_key_mapping_id();
    assert_eq!(VirtualKeyId::from_str("Control"), Ok(VirtualKeyId::Control));
}

#[test]
fn virtual_key_modifer() {
    let modifiers = VirtualKeyId::Control.modifier().unwrap().bits();
    let left_modifier = VirtualKeyId::ControlLeft.modifier().unwrap().bits();

    let right_alt = VirtualKeyId::AltRight.modifier().unwrap().bits();

    println!("{:b}", right_alt & !modifiers);

    // 0b0001_0001
    // 0b0001_0000
    println!("{:b}", !modifiers);

    assert_eq!(0, left_modifier & !modifiers);

    println!("{:b}", !VirtualKeyId::Control.modifier().unwrap().bits());

    assert_eq!(KeyModifiers::empty(), VirtualKeyId::ControlLeft.modifier().unwrap() & !VirtualKeyId::Control.modifier().unwrap());
}