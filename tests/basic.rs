use vin_info::{Vin, Region};

#[test]
fn should_verify_vin() {
    let vin = Vin::new("1M8GDM9AXKP042788");
    assert!(vin.calculate_checksum_digit() == 'X');
    assert!(vin.is_checksum_valid());
    assert_eq!(vin.wmi().len(), 3);
    assert_eq!(vin.wmi(), "1M8");
    assert_eq!(vin.vic(), "KP042788");
    assert_eq!(vin.manufacturer_region().unwrap(), Region::NorthAmerica);
    assert_eq!(vin.manufacturer_country(), "United States");
    assert_eq!(vin.manufacturer_name(), "Motor Coach Industries, Inc.");
}
