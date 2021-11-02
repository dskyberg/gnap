use get_if_addrs;

/// Get the machine IP Address
/// Get the IP from a non-loopback interface and return as a string.
pub fn get_machine_ip() -> String {
    let addrs = get_if_addrs::get_if_addrs().unwrap();
   let ips = addrs.into_iter()
    .filter(|n| n.name != "lo0")
    .collect::<Vec<_>>();

    format!(" {:?}", ips[0].addr.ip())
}
