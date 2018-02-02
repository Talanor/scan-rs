pub mod IPProtocol {
    pub const HOPOPT: u8 = 0;
    pub const ICMP: u8 = 1;
    pub const IGMP: u8 = 2;
    pub const GGP: u8 = 3;
    pub const IP_in_IP: u8 = 4;
    pub const ST: u8 = 5;
    pub const TCP: u8 = 6;
    pub const CBT: u8 = 7;
    pub const EGP: u8 = 8;
    pub const IGP: u8 = 9;
    pub const BBN_RCC_MON: u8 = 10;
    pub const NVP_II: u8 = 11;
    pub const PUP: u8 = 12;
    pub const ARGUS: u8 = 13;
    pub const EMCON: u8 = 14;
    pub const XNET: u8 = 15;
    pub const CHAOS: u8 = 16;
    pub const UDP: u8 = 17;
    pub const MUX: u8 = 18;
    pub const DCN_MEAS: u8 = 19;
    pub const HMP: u8 = 20;
    pub const PRM: u8 = 21;
    pub const XNS_IDP: u8 = 22;
    pub const TRUNK_1: u8 = 23;
    pub const TRUNK_2: u8 = 24;
    pub const LEAF_1: u8 = 25;
    pub const LEAF_2: u8 = 26;
    pub const RDP: u8 = 27;
    pub const IRTP: u8 = 28;
    pub const ISO_TP4: u8 = 29;
    pub const NETBLT: u8 = 30;
    pub const MFE_NSP: u8 = 31;
    pub const MERIT_INP: u8 = 32;
    pub const DCCP: u8 = 33;
    pub const 3PC: u8 = 34;
    pub const IDPR: u8 = 35;
    pub const XTP: u8 = 36;
    pub const DDP: u8 = 37;
    pub const IDPR_CMTP: u8 = 38;
    pub const TPPP: u8 = 39; // TP++
    pub const IL: u8 = 40;
    pub const IPv6: u8 = 41;
    pub const SRDP: u8 = 42;
    pub const IPv6_ROUTE: u8 = 43;
    pub const IPv6_FRAG: u8 = 44;
    pub const IDRP: u8 = 45;
    pub const RSVP: u8 = 46;
    pub const GREs: u8 = 47;
    pub const DSR: u8 = 48;
    pub const BNA: u8 = 49;
    pub const ESP: u8 = 50;
    pub const AH: u8 = 51;
    pub const I_NSLP: u8 = 52;
    pub const SWIPE: u8 = 53;
    pub const NARP: u8 = 54;
    pub const MOBILE: u8 = 55;
    pub const TLSP: u8 = 56;
    pub const SKIP: u8 = 57;
    pub const IPv6_ICMP: u8 = 58;
    pub const IPv6_NoNxt: u8 = 59;
    pub const IPv6_Opts: u8 = 60;
    //
    pub const CFTP: u8 = 62;
    //
    pub const SAT_EXPAK: u8 = 64;
    pub const KRYPTOLAN: u8 = 65;
    pub const RVD: u8 = 66;
    pub const IPPC: u8 = 67;
    //
    pub const SAT_MON: u8 = 69;
    pub const VISA: u8 = 70;
    pub const IPCU: u8 = 71;
    pub const CPNX: u8 = 72;
    pub const CPHB: u8 = 73;
    pub const WSN: u8 = 74;
    pub const PVP: u8 = 75;
    pub const BR_SAT_MON: u8 = 76;
    pub const SUN_ND: u8 = 77;
    pub const WB_MON: u8 = 78;
    pub const WB_EXPAK: u8 = 79;
    pub const ISO_IP: u8 = 80;
    pub const VMTP: u8 = 81;
    pub const SECURE_VMTP: u8 = 82;
    pub const VINES: u8 = 83;
    pub const TTP: u8 = 84;
    pub const IPTM: u8 = 84;
    pub const NSFNET_IGP: u8 = 85;
    pub const DGP: u8 = 86;
    pub const TCF: u8 = 87;
    pub const EIGRP: u8 = 88;
    pub const OSPF: u8 = 89;
    pub const Sprite_RPC: u8 = 90;
    pub const LARP: u8 = 91;
    pub const MTP: u8 = 92;
    pub const AX_25: u8 = 93;
    pub const OS: u8 = 94;
    pub const MICP: u8 = 95;
    pub const SCC_SP: u8 = 96;
    pub const ETHERIP: u8 = 97;
    pub const ENCAP: u8 = 98;
    //
    pub const GMTP: u8 = 100;
    pub const IFMP: u8 = 101;
    pub const PNNI: u8 = 102;
    pub const PIM: u8 = 103;
    pub const ARIS: u8 = 104;
    pub const SCPS: u8 = 105;
    pub const QNX: u8 = 106;
    pub const A_N: u8 = 107;
    pub const IPComp: u8 = 108;
    pub const SNP: u8 = 109;
    pub const Compaq_Peer: u8 = 110;
    pub const IPX_in_IP: u8 = 111;
    pub const VRRP: u8 = 112;
    pub const PGM: u8 = 113;
    //,
    pub const L2TP: u8 = 115;
    pub const DDX: u8 = 116;
    pub const IATP: u8 = 117;
    pub const STP: u8 = 118;
    pub const SRP: u8 = 119;
    pub const UTI: u8 = 120;
    pub const SMP: u8 = 121;
    pub const SM: u8 = 122;
    pub const PTP: u8 = 123;
    pub const IS_IS_over_IPv4: u8 = 124;
    pub const FIRE: u8 = 125;
    pub const CRTP: u8 = 126;
    pub const CRUDP: u8 = 127;
    pub const SSCOPMCE: u8 = 128;
    pub const IPLT: u8 = 129;
    pub const SPS: u8 = 130;
    pub const PIPE: u8 = 131;
    pub const SCTP: u8 = 132;
    pub const FC: u8 = 133;
    pub const RSVP_E2E_IGNORE: u8 = 134;
    pub const Mobility_Header: u8 = 135;
    pub const UDPLite: u8 = 136;
    pub const MPLS_in_IP: u8 = 137;
    pub const manet: u8 = 138;
    pub const HIP: u8 = 139;
    pub const Shim6: u8 = 140;
    pub const WESP: u8 = 141;
    pub const ROHC: u8 = 142;
}