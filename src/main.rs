extern crate nix;
extern crate tokio_core;
extern crate tun2tor;

use std::os::raw;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio_core::reactor::Core;

use tun2tor::io::stream_transfer;
use tun2tor::{DnsPortResolver, DnsTcpStack, SocksBackend, Tun};

#[no_mangle]
pub extern "C" fn connect(fd: raw::c_int, dnsgw_p: raw::c_ushort, socks_p: raw::c_ushort) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let utun = Tun::open(fd, &handle).unwrap();
    // utun.set_addr(Ipv4Addr::new(172, 30, 20, 1)).unwrap();
    // utun.set_netmask(Ipv4Addr::new(255, 255, 255, 255)).unwrap();

    let resolver = DnsPortResolver::new(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        dnsgw_p,
    ));
    let backend = SocksBackend::new(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        socks_p,
    ));
    let stack = DnsTcpStack::new(backend, resolver, &handle);

    core.run(stream_transfer(stack, utun)).unwrap();
}
