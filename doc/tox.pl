:- [ops].

path(["tox"], ["IpAddr"]) <: join([ref(path(["tox"], ["Ipv4Addr"])), ref(path(["tox"], ["Ipv6Addr"]))]).
path(["tox"], ["Ipv4Addr"]) <: prod([path(["tox"], ["Ipv4Addr", "0"]), path(["tox"], ["Ipv4Addr", "1"])]).
path(["tox"], ["Ipv4Addr", "0"]) <: meet([vint(0x02) ∈ integer, u(1, big_endian)]).
path(["tox"], ["Ipv4Addr", "1"]) <: arr(u(1, big_endian), [hole = rint(4)]).
path(["tox"], ["Ipv6Addr"]) <: prod([path(["tox"], ["Ipv6Addr", "0"]), path(["tox"], ["Ipv6Addr", "1"])]).
path(["tox"], ["Ipv6Addr", "0"]) <: meet([vint(0x0a) ∈ integer, u(1, big_endian)]).
path(["tox"], ["Ipv6Addr", "1"]) <: arr(u(1, big_endian), [hole = rint(16)]).
path(["tox"], ["NoSpam"]) <: arr(u(1, big_endian), [hole = rint(4)]).
path(["tox"], ["Nonce"]) <: arr(u(1, big_endian), [hole = rint(24)]).
path(["tox"], ["Packet"]) <: join([ref(path(["tox", "dht"], ["DhtPacket"])), ref(path(["tox", "onion"], ["OnionPacket"]))]).
path(["tox"], ["PublicKey"]) <: arr(u(1, big_endian), [hole = rint(32)]).
path(["tox"], ["Sha512"]) <: arr(u(1, big_endian), [hole = rint(64)]).
path(["tox", "dht"], ["BootstrapInfo"]) <: prod([path(["tox", "dht"], ["BootstrapInfo", "0"]), path(["tox", "dht"], ["BootstrapInfo", "1"]), path(["tox", "dht"], ["BootstrapInfo", "2"])]).
path(["tox", "dht"], ["BootstrapInfo", "0"]) <: meet([vint(0xf0) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["BootstrapInfo", "1"]) <: u(4, big_endian).
path(["tox", "dht"], ["BootstrapInfo", "2"]) <: arr(u(1, big_endian), [hole ≤ ref(path(["tox", "dht"], ["MaxMotdLength"]))]).
path(["tox", "dht"], ["Cookie"]) <: prod([path(["tox", "dht"], ["Cookie", "0"]), path(["tox", "dht"], ["Cookie", "1"]), path(["tox", "dht"], ["Cookie", "2"])]).
path(["tox", "dht"], ["Cookie", "0"]) <: u(8, big_endian).
path(["tox", "dht"], ["Cookie", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["Cookie", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["CookieRequest"]) <: prod([path(["tox", "dht"], ["CookieRequest", "0"]), path(["tox", "dht"], ["CookieRequest", "1"]), path(["tox", "dht"], ["CookieRequest", "2"]), path(["tox", "dht"], ["CookieRequest", "3"])]).
path(["tox", "dht"], ["CookieRequest", "0"]) <: meet([vint(0x18) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["CookieRequest", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["CookieRequest", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["CookieRequest", "3"]) <: arr(u(1, big_endian), [hole = rint(88)]).
path(["tox", "dht"], ["CookieRequestPayload"]) <: prod([path(["tox", "dht"], ["CookieRequestPayload", "0"]), path(["tox", "dht"], ["CookieRequestPayload", "1"]), path(["tox", "dht"], ["CookieRequestPayload", "2"])]).
path(["tox", "dht"], ["CookieRequestPayload", "0"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["CookieRequestPayload", "1"]) <: arr(vint(0) ∈ integer, [hole = rint(32)]).
path(["tox", "dht"], ["CookieRequestPayload", "2"]) <: u(8, big_endian).
path(["tox", "dht"], ["CookieResponse"]) <: prod([path(["tox", "dht"], ["CookieResponse", "0"]), path(["tox", "dht"], ["CookieResponse", "1"]), path(["tox", "dht"], ["CookieResponse", "2"])]).
path(["tox", "dht"], ["CookieResponse", "0"]) <: meet([vint(0x19) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["CookieResponse", "1"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["CookieResponse", "2"]) <: arr(u(1, big_endian), [hole = rint(136)]).
path(["tox", "dht"], ["CookieResponsePayload"]) <: prod([path(["tox", "dht"], ["CookieResponsePayload", "0"]), path(["tox", "dht"], ["CookieResponsePayload", "1"])]).
path(["tox", "dht"], ["CookieResponsePayload", "0"]) <: ref(path(["tox", "dht"], ["EncryptedCookie"])).
path(["tox", "dht"], ["CookieResponsePayload", "1"]) <: u(8, big_endian).
path(["tox", "dht"], ["CryptoData"]) <: prod([path(["tox", "dht"], ["CryptoData", "0"]), path(["tox", "dht"], ["CryptoData", "1"]), path(["tox", "dht"], ["CryptoData", "2"])]).
path(["tox", "dht"], ["CryptoData", "0"]) <: meet([vint(0x1b) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["CryptoData", "1"]) <: arr(u(1, big_endian), [hole = rint(2)]).
path(["tox", "dht"], ["CryptoData", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "dht"], ["CryptoDataPayload"]) <: prod([path(["tox", "dht"], ["CryptoDataPayload", "0"]), path(["tox", "dht"], ["CryptoDataPayload", "1"]), path(["tox", "dht"], ["CryptoDataPayload", "2"])]).
path(["tox", "dht"], ["CryptoDataPayload", "0"]) <: u(4, big_endian).
path(["tox", "dht"], ["CryptoDataPayload", "1"]) <: u(4, big_endian).
path(["tox", "dht"], ["CryptoDataPayload", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "dht"], ["CryptoHandshake"]) <: prod([path(["tox", "dht"], ["CryptoHandshake", "0"]), path(["tox", "dht"], ["CryptoHandshake", "1"]), path(["tox", "dht"], ["CryptoHandshake", "2"]), path(["tox", "dht"], ["CryptoHandshake", "3"])]).
path(["tox", "dht"], ["CryptoHandshake", "0"]) <: meet([vint(0x1a) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["CryptoHandshake", "1"]) <: ref(path(["tox", "dht"], ["EncryptedCookie"])).
path(["tox", "dht"], ["CryptoHandshake", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["CryptoHandshake", "3"]) <: arr(u(1, big_endian), [hole = rint(248)]).
path(["tox", "dht"], ["CryptoHandshakePayload"]) <: prod([path(["tox", "dht"], ["CryptoHandshakePayload", "0"]), path(["tox", "dht"], ["CryptoHandshakePayload", "1"]), path(["tox", "dht"], ["CryptoHandshakePayload", "2"]), path(["tox", "dht"], ["CryptoHandshakePayload", "3"])]).
path(["tox", "dht"], ["CryptoHandshakePayload", "0"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["CryptoHandshakePayload", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["CryptoHandshakePayload", "2"]) <: ref(path(["tox"], ["Sha512"])).
path(["tox", "dht"], ["CryptoHandshakePayload", "3"]) <: ref(path(["tox", "dht"], ["EncryptedCookie"])).
path(["tox", "dht"], ["DhtPacket"]) <: join([ref(path(["tox", "dht"], ["PingRequest"])), ref(path(["tox", "dht"], ["PingResponse"])), ref(path(["tox", "dht"], ["NodesRequest"])), ref(path(["tox", "dht"], ["NodesResponse"])), ref(path(["tox", "dht"], ["CookieRequest"])), ref(path(["tox", "dht"], ["CookieResponse"])), ref(path(["tox", "dht"], ["CryptoHandshake"])), ref(path(["tox", "dht"], ["CryptoData"])), ref(path(["tox", "dht"], ["DhtRequest"])), ref(path(["tox", "dht"], ["LanDiscovery"])), ref(path(["tox", "dht"], ["BootstrapInfo"]))]).
path(["tox", "dht"], ["DhtPkAnnounce"]) <: prod([path(["tox", "dht"], ["DhtPkAnnounce", "0"]), path(["tox", "dht"], ["DhtPkAnnounce", "1"]), path(["tox", "dht"], ["DhtPkAnnounce", "2"]), path(["tox", "dht"], ["DhtPkAnnounce", "3"])]).
path(["tox", "dht"], ["DhtPkAnnounce", "0"]) <: meet([vint(0x9c) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["DhtPkAnnounce", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["DhtPkAnnounce", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["DhtPkAnnounce", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "dht"], ["DhtRequest"]) <: prod([path(["tox", "dht"], ["DhtRequest", "0"]), path(["tox", "dht"], ["DhtRequest", "1"]), path(["tox", "dht"], ["DhtRequest", "2"]), path(["tox", "dht"], ["DhtRequest", "3"]), path(["tox", "dht"], ["DhtRequest", "4"])]).
path(["tox", "dht"], ["DhtRequest", "0"]) <: meet([vint(0x20) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["DhtRequest", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["DhtRequest", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["DhtRequest", "3"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["DhtRequest", "4"]) <: arr(u(1, big_endian), []).
path(["tox", "dht"], ["DhtRequestPayload"]) <: join([ref(path(["tox", "dht"], ["NatPingRequest"])), ref(path(["tox", "dht"], ["NatPingResponse"])), ref(path(["tox", "dht"], ["DhtPkAnnounce"])), ref(path(["tox", "dht"], ["HardeningRequest"])), ref(path(["tox", "dht"], ["HardeningResponse"]))]).
path(["tox", "dht"], ["EncryptedCookie"]) <: prod([path(["tox", "dht"], ["EncryptedCookie", "0"]), path(["tox", "dht"], ["EncryptedCookie", "1"])]).
path(["tox", "dht"], ["EncryptedCookie", "0"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["EncryptedCookie", "1"]) <: arr(u(1, big_endian), [hole = rint(88)]).
path(["tox", "dht"], ["HardeningRequest"]) <: prod([path(["tox", "dht"], ["HardeningRequest", "0"]), path(["tox", "dht"], ["HardeningRequest", "1"]), path(["tox", "dht"], ["HardeningRequest", "2"])]).
path(["tox", "dht"], ["HardeningRequest", "0"]) <: meet([vint(0x30) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["HardeningRequest", "1"]) <: meet([vint(0x02) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["HardeningRequest", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "dht"], ["HardeningResponse"]) <: prod([path(["tox", "dht"], ["HardeningResponse", "0"]), path(["tox", "dht"], ["HardeningResponse", "1"]), path(["tox", "dht"], ["HardeningResponse", "2"])]).
path(["tox", "dht"], ["HardeningResponse", "0"]) <: meet([vint(0x30) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["HardeningResponse", "1"]) <: meet([vint(0x03) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["HardeningResponse", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "dht"], ["LanDiscovery"]) <: prod([path(["tox", "dht"], ["LanDiscovery", "0"]), path(["tox", "dht"], ["LanDiscovery", "1"])]).
path(["tox", "dht"], ["LanDiscovery", "0"]) <: meet([vint(0x21) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["LanDiscovery", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["MaxMotdLength"]) <: vint(256) ∈ integer.
path(["tox", "dht"], ["NatPingRequest"]) <: prod([path(["tox", "dht"], ["NatPingRequest", "0"]), path(["tox", "dht"], ["NatPingRequest", "1"]), path(["tox", "dht"], ["NatPingRequest", "2"])]).
path(["tox", "dht"], ["NatPingRequest", "0"]) <: meet([vint(0xfe) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["NatPingRequest", "1"]) <: meet([vint(0x00) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["NatPingRequest", "2"]) <: u(8, big_endian).
path(["tox", "dht"], ["NatPingResponse"]) <: prod([path(["tox", "dht"], ["NatPingResponse", "0"]), path(["tox", "dht"], ["NatPingResponse", "1"]), path(["tox", "dht"], ["NatPingResponse", "2"])]).
path(["tox", "dht"], ["NatPingResponse", "0"]) <: meet([vint(0xfe) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["NatPingResponse", "1"]) <: meet([vint(0x01) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["NatPingResponse", "2"]) <: u(8, big_endian).
path(["tox", "dht"], ["NodesRequest"]) <: prod([path(["tox", "dht"], ["NodesRequest", "0"]), path(["tox", "dht"], ["NodesRequest", "1"]), path(["tox", "dht"], ["NodesRequest", "2"])]).
path(["tox", "dht"], ["NodesRequest", "0"]) <: meet([vint(0x03) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["NodesRequest", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["NodesRequest", "2"]) <: arr(u(1, big_endian), [hole = rint(56)]).
path(["tox", "dht"], ["NodesRequestPayload"]) <: prod([path(["tox", "dht"], ["NodesRequestPayload", "0"]), path(["tox", "dht"], ["NodesRequestPayload", "1"])]).
path(["tox", "dht"], ["NodesRequestPayload", "0"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["NodesRequestPayload", "1"]) <: u(8, big_endian).
path(["tox", "dht"], ["NodesResponse"]) <: prod([path(["tox", "dht"], ["NodesResponse", "0"]), path(["tox", "dht"], ["NodesResponse", "1"]), path(["tox", "dht"], ["NodesResponse", "2"]), path(["tox", "dht"], ["NodesResponse", "3"]), path(["tox", "dht"], ["NodesResponse", "4"])]).
path(["tox", "dht"], ["NodesResponse", "0"]) <: meet([vint(0x04) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["NodesResponse", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["NodesResponse", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["NodesResponse", "3"]) <: u(1, big_endian).
path(["tox", "dht"], ["NodesResponse", "4"]) <: arr(u(1, big_endian), [hole ≥ rint(25), hole ≤ rint(229)]).
path(["tox", "dht"], ["NodesResponsePayload"]) <: prod([path(["tox", "dht"], ["NodesResponsePayload", "0"]), path(["tox", "dht"], ["NodesResponsePayload", "1"]), path(["tox", "dht"], ["NodesResponsePayload", "2"])]).
path(["tox", "dht"], ["NodesResponsePayload", "0"]) <: meet([top ⋮ [hole ≤ rint(4)], u(1, big_endian)]).
path(["tox", "dht"], ["NodesResponsePayload", "1"]) <: meet([top ⋮ [sizeof = size([hole ≤ rint(204)])], arr(ref(path(["tox", "dht"], ["PackedNode"])), [hole = ref(path(["tox", "dht"], ["NodesResponsePayload", "_number"]))])]).
path(["tox", "dht"], ["NodesResponsePayload", "2"]) <: u(8, big_endian).
path(["tox", "dht"], ["PackedNode"]) <: prod([path(["tox", "dht"], ["PackedNode", "0"]), path(["tox", "dht"], ["PackedNode", "1"]), path(["tox", "dht"], ["PackedNode", "2"])]).
path(["tox", "dht"], ["PackedNode", "0"]) <: ref(path(["tox"], ["IpAddr"])).
path(["tox", "dht"], ["PackedNode", "1"]) <: u(2, big_endian).
path(["tox", "dht"], ["PackedNode", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["PingRequest"]) <: prod([path(["tox", "dht"], ["PingRequest", "0"]), path(["tox", "dht"], ["PingRequest", "1"]), path(["tox", "dht"], ["PingRequest", "2"]), path(["tox", "dht"], ["PingRequest", "3"])]).
path(["tox", "dht"], ["PingRequest", "0"]) <: meet([vint(0x00) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["PingRequest", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["PingRequest", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["PingRequest", "3"]) <: arr(u(1, big_endian), [hole = rint(24)]).
path(["tox", "dht"], ["PingRequestPayload"]) <: prod([path(["tox", "dht"], ["PingRequestPayload", "0"]), path(["tox", "dht"], ["PingRequestPayload", "1"])]).
path(["tox", "dht"], ["PingRequestPayload", "0"]) <: meet([vint(0x00) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["PingRequestPayload", "1"]) <: u(8, big_endian).
path(["tox", "dht"], ["PingResponse"]) <: prod([path(["tox", "dht"], ["PingResponse", "0"]), path(["tox", "dht"], ["PingResponse", "1"]), path(["tox", "dht"], ["PingResponse", "2"]), path(["tox", "dht"], ["PingResponse", "3"])]).
path(["tox", "dht"], ["PingResponse", "0"]) <: meet([vint(0x01) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["PingResponse", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "dht"], ["PingResponse", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "dht"], ["PingResponse", "3"]) <: arr(u(1, big_endian), [hole = rint(24)]).
path(["tox", "dht"], ["PingResponsePayload"]) <: prod([path(["tox", "dht"], ["PingResponsePayload", "0"]), path(["tox", "dht"], ["PingResponsePayload", "1"])]).
path(["tox", "dht"], ["PingResponsePayload", "0"]) <: meet([vint(0x01) ∈ integer, u(1, big_endian)]).
path(["tox", "dht"], ["PingResponsePayload", "1"]) <: u(8, big_endian).
path(["tox", "friend_connection"], ["Alive"]) <: prod([path(["tox", "friend_connection"], ["Alive", "0"])]).
path(["tox", "friend_connection"], ["Alive", "0"]) <: meet([vint(0x10) ∈ integer, u(1, big_endian)]).
path(["tox", "friend_connection"], ["FcPacket"]) <: join([ref(path(["tox", "friend_connection"], ["Alive"])), ref(path(["tox", "friend_connection"], ["ShareRelays"])), ref(path(["tox", "friend_connection"], ["FriendRequests"]))]).
path(["tox", "friend_connection"], ["FriendRequests"]) <: prod([path(["tox", "friend_connection"], ["FriendRequests", "0"]), path(["tox", "friend_connection"], ["FriendRequests", "1"]), path(["tox", "friend_connection"], ["FriendRequests", "2"])]).
path(["tox", "friend_connection"], ["FriendRequests", "0"]) <: meet([vint(0x12) ∈ integer, u(1, big_endian)]).
path(["tox", "friend_connection"], ["FriendRequests", "1"]) <: ref(path(["tox"], ["NoSpam"])).
path(["tox", "friend_connection"], ["FriendRequests", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "friend_connection"], ["ShareRelays"]) <: prod([path(["tox", "friend_connection"], ["ShareRelays", "0"]), path(["tox", "friend_connection"], ["ShareRelays", "1"])]).
path(["tox", "friend_connection"], ["ShareRelays", "0"]) <: meet([vint(0x11) ∈ integer, u(1, big_endian)]).
path(["tox", "friend_connection"], ["ShareRelays", "1"]) <: meet([top ⋮ [sizeof = size([hole ≤ rint(153)])], arr(ref(path(["tox", "dht"], ["PackedNode"])), [])]).
path(["tox", "onion"], ["AnnounceRequest"]) <: prod([path(["tox", "onion"], ["AnnounceRequest", "0"]), path(["tox", "onion"], ["AnnounceRequest", "1"]), path(["tox", "onion"], ["AnnounceRequest", "2"]), path(["tox", "onion"], ["AnnounceRequest", "3"])]).
path(["tox", "onion"], ["AnnounceRequest", "0"]) <: meet([vint(0x83) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["AnnounceRequest", "1"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["AnnounceRequest", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["AnnounceRequest", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["AnnounceResponse"]) <: prod([path(["tox", "onion"], ["AnnounceResponse", "0"]), path(["tox", "onion"], ["AnnounceResponse", "1"]), path(["tox", "onion"], ["AnnounceResponse", "2"]), path(["tox", "onion"], ["AnnounceResponse", "3"])]).
path(["tox", "onion"], ["AnnounceResponse", "0"]) <: meet([vint(0x84) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["AnnounceResponse", "1"]) <: u(8, big_endian).
path(["tox", "onion"], ["AnnounceResponse", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["AnnounceResponse", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionDataRequest"]) <: prod([path(["tox", "onion"], ["OnionDataRequest", "0"]), path(["tox", "onion"], ["OnionDataRequest", "1"]), path(["tox", "onion"], ["OnionDataRequest", "2"]), path(["tox", "onion"], ["OnionDataRequest", "3"]), path(["tox", "onion"], ["OnionDataRequest", "4"])]).
path(["tox", "onion"], ["OnionDataRequest", "0"]) <: meet([vint(0x85) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionDataRequest", "1"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["OnionDataRequest", "2"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["OnionDataRequest", "3"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["OnionDataRequest", "4"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionDataResponse"]) <: prod([path(["tox", "onion"], ["OnionDataResponse", "0"]), path(["tox", "onion"], ["OnionDataResponse", "1"]), path(["tox", "onion"], ["OnionDataResponse", "2"]), path(["tox", "onion"], ["OnionDataResponse", "3"])]).
path(["tox", "onion"], ["OnionDataResponse", "0"]) <: meet([vint(0x86) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionDataResponse", "1"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["OnionDataResponse", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["OnionDataResponse", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionPacket"]) <: join([ref(path(["tox", "onion"], ["OnionRequest0"])), ref(path(["tox", "onion"], ["OnionRequest1"])), ref(path(["tox", "onion"], ["OnionRequest2"])), ref(path(["tox", "onion"], ["AnnounceRequest"])), ref(path(["tox", "onion"], ["AnnounceResponse"])), ref(path(["tox", "onion"], ["OnionDataRequest"])), ref(path(["tox", "onion"], ["OnionDataResponse"])), ref(path(["tox", "onion"], ["OnionResponse3"])), ref(path(["tox", "onion"], ["OnionResponse2"])), ref(path(["tox", "onion"], ["OnionResponse1"]))]).
path(["tox", "onion"], ["OnionRequest0"]) <: prod([path(["tox", "onion"], ["OnionRequest0", "0"]), path(["tox", "onion"], ["OnionRequest0", "1"]), path(["tox", "onion"], ["OnionRequest0", "2"]), path(["tox", "onion"], ["OnionRequest0", "3"])]).
path(["tox", "onion"], ["OnionRequest0", "0"]) <: meet([vint(0x80) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionRequest0", "1"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["OnionRequest0", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["OnionRequest0", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionRequest1"]) <: prod([path(["tox", "onion"], ["OnionRequest1", "0"]), path(["tox", "onion"], ["OnionRequest1", "1"]), path(["tox", "onion"], ["OnionRequest1", "2"]), path(["tox", "onion"], ["OnionRequest1", "3"]), path(["tox", "onion"], ["OnionRequest1", "4"])]).
path(["tox", "onion"], ["OnionRequest1", "0"]) <: meet([vint(0x81) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionRequest1", "1"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["OnionRequest1", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["OnionRequest1", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionRequest1", "4"]) <: arr(u(1, big_endian), [hole = rint(59)]).
path(["tox", "onion"], ["OnionRequest2"]) <: prod([path(["tox", "onion"], ["OnionRequest2", "0"]), path(["tox", "onion"], ["OnionRequest2", "1"]), path(["tox", "onion"], ["OnionRequest2", "2"]), path(["tox", "onion"], ["OnionRequest2", "3"]), path(["tox", "onion"], ["OnionRequest2", "4"])]).
path(["tox", "onion"], ["OnionRequest2", "0"]) <: meet([vint(0x82) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionRequest2", "1"]) <: ref(path(["tox"], ["Nonce"])).
path(["tox", "onion"], ["OnionRequest2", "2"]) <: ref(path(["tox"], ["PublicKey"])).
path(["tox", "onion"], ["OnionRequest2", "3"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionRequest2", "4"]) <: arr(u(1, big_endian), [hole = rint(118)]).
path(["tox", "onion"], ["OnionResponse1"]) <: prod([path(["tox", "onion"], ["OnionResponse1", "0"]), path(["tox", "onion"], ["OnionResponse1", "1"]), path(["tox", "onion"], ["OnionResponse1", "2"])]).
path(["tox", "onion"], ["OnionResponse1", "0"]) <: meet([vint(0x8e) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionResponse1", "1"]) <: arr(u(1, big_endian), [hole = rint(59)]).
path(["tox", "onion"], ["OnionResponse1", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionResponse2"]) <: prod([path(["tox", "onion"], ["OnionResponse2", "0"]), path(["tox", "onion"], ["OnionResponse2", "1"]), path(["tox", "onion"], ["OnionResponse2", "2"])]).
path(["tox", "onion"], ["OnionResponse2", "0"]) <: meet([vint(0x8d) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionResponse2", "1"]) <: arr(u(1, big_endian), [hole = rint(118)]).
path(["tox", "onion"], ["OnionResponse2", "2"]) <: arr(u(1, big_endian), []).
path(["tox", "onion"], ["OnionResponse3"]) <: prod([path(["tox", "onion"], ["OnionResponse3", "0"]), path(["tox", "onion"], ["OnionResponse3", "1"]), path(["tox", "onion"], ["OnionResponse3", "2"])]).
path(["tox", "onion"], ["OnionResponse3", "0"]) <: meet([vint(0x8c) ∈ integer, u(1, big_endian)]).
path(["tox", "onion"], ["OnionResponse3", "1"]) <: arr(u(1, big_endian), [hole = rint(177)]).
path(["tox", "onion"], ["OnionResponse3", "2"]) <: arr(u(1, big_endian), []).
