//! Wrapper for DPDK's rte_flow.
use std::borrow::BorrowMut;

use crate::ffi;
use crate::zeroable::Zeroable;

#[doc = " [META]"]
#[doc = ""]
#[doc = " End marker for item lists. Prevents further processing of items,"]
#[doc = " thereby ending the pattern."]
#[doc = ""]
#[doc = " No associated specification structure."]
pub const RTE_FLOW_ITEM_TYPE_END: RteFlowItemType = 0;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Used as a placeholder for convenience. It is ignored and simply"]
#[doc = " discarded by PMDs."]
#[doc = ""]
#[doc = " No associated specification structure."]
pub const RTE_FLOW_ITEM_TYPE_VOID: RteFlowItemType = 1;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Inverted matching, i.e. process packets that do not match the"]
#[doc = " pattern."]
#[doc = ""]
#[doc = " No associated specification structure."]
pub const RTE_FLOW_ITEM_TYPE_INVERT: RteFlowItemType = 2;
#[doc = " Matches any protocol in place of the current layer, a single ANY"]
#[doc = " may also stand for several protocol layers."]
#[doc = ""]
#[doc = " See struct rte_flow_item_any."]
pub const RTE_FLOW_ITEM_TYPE_ANY: RteFlowItemType = 3;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Matches traffic originating from (ingress) or going to (egress)"]
#[doc = " the physical function of the current device."]
#[doc = ""]
#[doc = " No associated specification structure."]
pub const RTE_FLOW_ITEM_TYPE_PF: RteFlowItemType = 4;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Matches traffic originating from (ingress) or going to (egress) a"]
#[doc = " given virtual function of the current device."]
#[doc = ""]
#[doc = " See struct rte_flow_item_vf."]
pub const RTE_FLOW_ITEM_TYPE_VF: RteFlowItemType = 5;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Matches traffic originating from (ingress) or going to (egress) a"]
#[doc = " physical port of the underlying device."]
#[doc = ""]
#[doc = " See struct rte_flow_item_phy_port."]
pub const RTE_FLOW_ITEM_TYPE_PHY_PORT: RteFlowItemType = 6;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Matches traffic originating from (ingress) or going to (egress) a"]
#[doc = " given DPDK port ID."]
#[doc = ""]
#[doc = " See struct rte_flow_item_port_id."]
pub const RTE_FLOW_ITEM_TYPE_PORT_ID: RteFlowItemType = 7;
#[doc = " Matches a byte string of a given length at a given offset."]
#[doc = ""]
#[doc = " See struct rte_flow_item_raw."]
pub const RTE_FLOW_ITEM_TYPE_RAW: RteFlowItemType = 8;
#[doc = " Matches an Ethernet header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_eth."]
pub const RTE_FLOW_ITEM_TYPE_ETH: RteFlowItemType = 9;
#[doc = " Matches an 802.1Q/ad VLAN tag."]
#[doc = ""]
#[doc = " See struct rte_flow_item_vlan."]
pub const RTE_FLOW_ITEM_TYPE_VLAN: RteFlowItemType = 10;
#[doc = " Matches an IPv4 header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_ipv4."]
pub const RTE_FLOW_ITEM_TYPE_IPV4: RteFlowItemType = 11;
#[doc = " Matches an IPv6 header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_ipv6."]
pub const RTE_FLOW_ITEM_TYPE_IPV6: RteFlowItemType = 12;
#[doc = " Matches an ICMP header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp."]
pub const RTE_FLOW_ITEM_TYPE_ICMP: RteFlowItemType = 13;
#[doc = " Matches a UDP header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_udp."]
pub const RTE_FLOW_ITEM_TYPE_UDP: RteFlowItemType = 14;
#[doc = " Matches a TCP header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_tcp."]
pub const RTE_FLOW_ITEM_TYPE_TCP: RteFlowItemType = 15;
#[doc = " Matches a SCTP header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_sctp."]
pub const RTE_FLOW_ITEM_TYPE_SCTP: RteFlowItemType = 16;
#[doc = " Matches a VXLAN header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_vxlan."]
pub const RTE_FLOW_ITEM_TYPE_VXLAN: RteFlowItemType = 17;
#[doc = " Matches a E_TAG header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_e_tag."]
pub const RTE_FLOW_ITEM_TYPE_E_TAG: RteFlowItemType = 18;
#[doc = " Matches a NVGRE header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_nvgre."]
pub const RTE_FLOW_ITEM_TYPE_NVGRE: RteFlowItemType = 19;
#[doc = " Matches a MPLS header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_mpls."]
pub const RTE_FLOW_ITEM_TYPE_MPLS: RteFlowItemType = 20;
#[doc = " Matches a GRE header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_gre."]
pub const RTE_FLOW_ITEM_TYPE_GRE: RteFlowItemType = 21;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Fuzzy pattern match, expect faster than default."]
#[doc = ""]
#[doc = " This is for device that support fuzzy matching option."]
#[doc = " Usually a fuzzy matching is fast but the cost is accuracy."]
#[doc = ""]
#[doc = " See struct rte_flow_item_fuzzy."]
pub const RTE_FLOW_ITEM_TYPE_FUZZY: RteFlowItemType = 22;
#[doc = " Matches a GTP header."]
#[doc = ""]
#[doc = " Configure flow for GTP packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_gtp."]
pub const RTE_FLOW_ITEM_TYPE_GTP: RteFlowItemType = 23;
#[doc = " Matches a GTP header."]
#[doc = ""]
#[doc = " Configure flow for GTP-C packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_gtp."]
pub const RTE_FLOW_ITEM_TYPE_GTPC: RteFlowItemType = 24;
#[doc = " Matches a GTP header."]
#[doc = ""]
#[doc = " Configure flow for GTP-U packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_gtp."]
pub const RTE_FLOW_ITEM_TYPE_GTPU: RteFlowItemType = 25;
#[doc = " Matches a ESP header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_esp."]
pub const RTE_FLOW_ITEM_TYPE_ESP: RteFlowItemType = 26;
#[doc = " Matches a GENEVE header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_geneve."]
pub const RTE_FLOW_ITEM_TYPE_GENEVE: RteFlowItemType = 27;
#[doc = " Matches a VXLAN-GPE header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_vxlan_gpe."]
pub const RTE_FLOW_ITEM_TYPE_VXLAN_GPE: RteFlowItemType = 28;
#[doc = " Matches an ARP header for Ethernet/IPv4."]
#[doc = ""]
#[doc = " See struct rte_flow_item_arp_eth_ipv4."]
pub const RTE_FLOW_ITEM_TYPE_ARP_ETH_IPV4: RteFlowItemType = 29;
#[doc = " Matches the presence of any IPv6 extension header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_ipv6_ext."]
pub const RTE_FLOW_ITEM_TYPE_IPV6_EXT: RteFlowItemType = 30;
#[doc = " Matches any ICMPv6 header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp6."]
pub const RTE_FLOW_ITEM_TYPE_ICMP6: RteFlowItemType = 31;
#[doc = " Matches an ICMPv6 neighbor discovery solicitation."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp6_nd_ns."]
pub const RTE_FLOW_ITEM_TYPE_ICMP6_ND_NS: RteFlowItemType = 32;
#[doc = " Matches an ICMPv6 neighbor discovery advertisement."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp6_nd_na."]
pub const RTE_FLOW_ITEM_TYPE_ICMP6_ND_NA: RteFlowItemType = 33;
#[doc = " Matches the presence of any ICMPv6 neighbor discovery option."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp6_nd_opt."]
pub const RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT: RteFlowItemType = 34;
#[doc = " Matches an ICMPv6 neighbor discovery source Ethernet link-layer"]
#[doc = " address option."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp6_nd_opt_sla_eth."]
pub const RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT_SLA_ETH: RteFlowItemType = 35;
#[doc = " Matches an ICMPv6 neighbor discovery target Ethernet link-layer"]
#[doc = " address option."]
#[doc = ""]
#[doc = " See struct rte_flow_item_icmp6_nd_opt_tla_eth."]
pub const RTE_FLOW_ITEM_TYPE_ICMP6_ND_OPT_TLA_ETH: RteFlowItemType = 36;
#[doc = " Matches specified mark field."]
#[doc = ""]
#[doc = " See struct rte_flow_item_mark."]
pub const RTE_FLOW_ITEM_TYPE_MARK: RteFlowItemType = 37;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Matches a metadata value."]
#[doc = ""]
#[doc = " See struct rte_flow_item_meta."]
pub const RTE_FLOW_ITEM_TYPE_META: RteFlowItemType = 38;
#[doc = " Matches a GRE optional key field."]
#[doc = ""]
#[doc = " The value should a big-endian 32bit integer."]
#[doc = ""]
#[doc = " When this item present the K bit is implicitly matched as \"1\""]
#[doc = " in the default mask."]
#[doc = ""]
#[doc = " @p spec/mask type:"]
#[doc = " @code rte_be32_t * @endcode"]
pub const RTE_FLOW_ITEM_TYPE_GRE_KEY: RteFlowItemType = 39;
#[doc = " Matches a GTP extension header: PDU session container."]
#[doc = ""]
#[doc = " Configure flow for GTP packets with extension header type 0x85."]
#[doc = ""]
#[doc = " See struct rte_flow_item_gtp_psc."]
pub const RTE_FLOW_ITEM_TYPE_GTP_PSC: RteFlowItemType = 40;
#[doc = " Matches a PPPoE header."]
#[doc = ""]
#[doc = " Configure flow for PPPoE session packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_pppoe."]
pub const RTE_FLOW_ITEM_TYPE_PPPOES: RteFlowItemType = 41;
#[doc = " Matches a PPPoE header."]
#[doc = ""]
#[doc = " Configure flow for PPPoE discovery packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_pppoe."]
pub const RTE_FLOW_ITEM_TYPE_PPPOED: RteFlowItemType = 42;
#[doc = " Matches a PPPoE optional proto_id field."]
#[doc = ""]
#[doc = " It only applies to PPPoE session packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_pppoe_proto_id."]
pub const RTE_FLOW_ITEM_TYPE_PPPOE_PROTO_ID: RteFlowItemType = 43;
#[doc = " Matches Network service header (NSH)."]
#[doc = " See struct rte_flow_item_nsh."]
#[doc = ""]
pub const RTE_FLOW_ITEM_TYPE_NSH: RteFlowItemType = 44;
#[doc = " Matches Internet Group Management Protocol (IGMP)."]
#[doc = " See struct rte_flow_item_igmp."]
#[doc = ""]
pub const RTE_FLOW_ITEM_TYPE_IGMP: RteFlowItemType = 45;
#[doc = " Matches IP Authentication Header (AH)."]
#[doc = " See struct rte_flow_item_ah."]
#[doc = ""]
pub const RTE_FLOW_ITEM_TYPE_AH: RteFlowItemType = 46;
#[doc = " Matches a HIGIG header."]
#[doc = " see struct rte_flow_item_higig2_hdr."]
pub const RTE_FLOW_ITEM_TYPE_HIGIG2: RteFlowItemType = 47;
#[doc = " [META]"]
#[doc = ""]
#[doc = " Matches a tag value."]
#[doc = ""]
#[doc = " See struct rte_flow_item_tag."]
pub const RTE_FLOW_ITEM_TYPE_TAG: RteFlowItemType = 48;
#[doc = " Matches a L2TPv3 over IP header."]
#[doc = ""]
#[doc = " Configure flow for L2TPv3 over IP packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_l2tpv3oip."]
pub const RTE_FLOW_ITEM_TYPE_L2TPV3OIP: RteFlowItemType = 49;
#[doc = " Matches PFCP Header."]
#[doc = " See struct rte_flow_item_pfcp."]
#[doc = ""]
pub const RTE_FLOW_ITEM_TYPE_PFCP: RteFlowItemType = 50;
#[doc = " Matches eCPRI Header."]
#[doc = ""]
#[doc = " Configure flow for eCPRI over ETH or UDP packets."]
#[doc = ""]
#[doc = " See struct rte_flow_item_ecpri."]
pub const RTE_FLOW_ITEM_TYPE_ECPRI: RteFlowItemType = 51;
#[doc = " Matches the presence of IPv6 fragment extension header."]
#[doc = ""]
#[doc = " See struct rte_flow_item_ipv6_frag_ext."]
pub const RTE_FLOW_ITEM_TYPE_IPV6_FRAG_EXT: RteFlowItemType = 52;

pub type RteFlowItemType = dpdk_sys::rte_flow_item_type;

#[doc = " End marker for action lists. Prevents further processing of"]
#[doc = " actions, thereby ending the list."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_END: RteFlowActionType = 0;
#[doc = " Used as a placeholder for convenience. It is ignored and simply"]
#[doc = " discarded by PMDs."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_VOID: RteFlowActionType = 1;
#[doc = " Leaves traffic up for additional processing by subsequent flow"]
#[doc = " rules; makes a flow rule non-terminating."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_PASSTHRU: RteFlowActionType = 2;
#[doc = " RTE_FLOW_ACTION_TYPE_JUMP"]
#[doc = ""]
#[doc = " Redirects packets to a group on the current device."]
#[doc = ""]
#[doc = " See struct rte_flow_action_jump."]
pub const RTE_FLOW_ACTION_TYPE_JUMP: RteFlowActionType = 3;
#[doc = " Attaches an integer value to packets and sets PKT_RX_FDIR and"]
#[doc = " PKT_RX_FDIR_ID mbuf flags."]
#[doc = ""]
#[doc = " See struct rte_flow_action_mark."]
pub const RTE_FLOW_ACTION_TYPE_MARK: RteFlowActionType = 4;
#[doc = " Flags packets. Similar to MARK without a specific value; only"]
#[doc = " sets the PKT_RX_FDIR mbuf flag."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_FLAG: RteFlowActionType = 5;
#[doc = " Assigns packets to a given queue index."]
#[doc = ""]
#[doc = " See struct rte_flow_action_queue."]
pub const RTE_FLOW_ACTION_TYPE_QUEUE: RteFlowActionType = 6;
#[doc = " Drops packets."]
#[doc = ""]
#[doc = " PASSTHRU overrides this action if both are specified."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_DROP: RteFlowActionType = 7;
#[doc = " Enables counters for this flow rule."]
#[doc = ""]
#[doc = " These counters can be retrieved and reset through rte_flow_query() or"]
#[doc = " rte_flow_shared_action_query() if the action provided via handle,"]
#[doc = " see struct rte_flow_query_count."]
#[doc = ""]
#[doc = " See struct rte_flow_action_count."]
pub const RTE_FLOW_ACTION_TYPE_COUNT: RteFlowActionType = 8;
#[doc = " Similar to QUEUE, except RSS is additionally performed on packets"]
#[doc = " to spread them among several queues according to the provided"]
#[doc = " parameters."]
#[doc = ""]
#[doc = " See struct rte_flow_action_rss."]
pub const RTE_FLOW_ACTION_TYPE_RSS: RteFlowActionType = 9;
#[doc = " Directs matching traffic to the physical function (PF) of the"]
#[doc = " current device."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_PF: RteFlowActionType = 10;
#[doc = " Directs matching traffic to a given virtual function of the"]
#[doc = " current device."]
#[doc = ""]
#[doc = " See struct rte_flow_action_vf."]
pub const RTE_FLOW_ACTION_TYPE_VF: RteFlowActionType = 11;
#[doc = " Directs packets to a given physical port index of the underlying"]
#[doc = " device."]
#[doc = ""]
#[doc = " See struct rte_flow_action_phy_port."]
pub const RTE_FLOW_ACTION_TYPE_PHY_PORT: RteFlowActionType = 12;
#[doc = " Directs matching traffic to a given DPDK port ID."]
#[doc = ""]
#[doc = " See struct rte_flow_action_port_id."]
pub const RTE_FLOW_ACTION_TYPE_PORT_ID: RteFlowActionType = 13;
#[doc = " Traffic metering and policing (MTR)."]
#[doc = ""]
#[doc = " See struct rte_flow_action_meter."]
#[doc = " See file rte_mtr.h for MTR object configuration."]
pub const RTE_FLOW_ACTION_TYPE_METER: RteFlowActionType = 14;
#[doc = " Redirects packets to security engine of current device for security"]
#[doc = " processing as specified by security session."]
#[doc = ""]
#[doc = " See struct rte_flow_action_security."]
pub const RTE_FLOW_ACTION_TYPE_SECURITY: RteFlowActionType = 15;
#[doc = " Implements OFPAT_SET_MPLS_TTL (\"MPLS TTL\") as defined by the"]
#[doc = " OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_set_mpls_ttl."]
pub const RTE_FLOW_ACTION_TYPE_OF_SET_MPLS_TTL: RteFlowActionType = 16;
#[doc = " Implements OFPAT_DEC_MPLS_TTL (\"decrement MPLS TTL\") as defined"]
#[doc = " by the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_OF_DEC_MPLS_TTL: RteFlowActionType = 17;
#[doc = " Implements OFPAT_SET_NW_TTL (\"IP TTL\") as defined by the OpenFlow"]
#[doc = " Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_set_nw_ttl."]
pub const RTE_FLOW_ACTION_TYPE_OF_SET_NW_TTL: RteFlowActionType = 18;
#[doc = " Implements OFPAT_DEC_NW_TTL (\"decrement IP TTL\") as defined by"]
#[doc = " the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_OF_DEC_NW_TTL: RteFlowActionType = 19;
#[doc = " Implements OFPAT_COPY_TTL_OUT (\"copy TTL \"outwards\" -- from"]
#[doc = " next-to-outermost to outermost\") as defined by the OpenFlow"]
#[doc = " Switch Specification."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_OF_COPY_TTL_OUT: RteFlowActionType = 20;
#[doc = " Implements OFPAT_COPY_TTL_IN (\"copy TTL \"inwards\" -- from"]
#[doc = " outermost to next-to-outermost\") as defined by the OpenFlow"]
#[doc = " Switch Specification."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_OF_COPY_TTL_IN: RteFlowActionType = 21;
#[doc = " Implements OFPAT_POP_VLAN (\"pop the outer VLAN tag\") as defined"]
#[doc = " by the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_OF_POP_VLAN: RteFlowActionType = 22;
#[doc = " Implements OFPAT_PUSH_VLAN (\"push a new VLAN tag\") as defined by"]
#[doc = " the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_push_vlan."]
pub const RTE_FLOW_ACTION_TYPE_OF_PUSH_VLAN: RteFlowActionType = 23;
#[doc = " Implements OFPAT_SET_VLAN_VID (\"set the 802.1q VLAN id\") as"]
#[doc = " defined by the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_set_vlan_vid."]
pub const RTE_FLOW_ACTION_TYPE_OF_SET_VLAN_VID: RteFlowActionType = 24;
#[doc = " Implements OFPAT_SET_LAN_PCP (\"set the 802.1q priority\") as"]
#[doc = " defined by the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_set_vlan_pcp."]
pub const RTE_FLOW_ACTION_TYPE_OF_SET_VLAN_PCP: RteFlowActionType = 25;
#[doc = " Implements OFPAT_POP_MPLS (\"pop the outer MPLS tag\") as defined"]
#[doc = " by the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_pop_mpls."]
pub const RTE_FLOW_ACTION_TYPE_OF_POP_MPLS: RteFlowActionType = 26;
#[doc = " Implements OFPAT_PUSH_MPLS (\"push a new MPLS tag\") as defined by"]
#[doc = " the OpenFlow Switch Specification."]
#[doc = ""]
#[doc = " See struct rte_flow_action_of_push_mpls."]
pub const RTE_FLOW_ACTION_TYPE_OF_PUSH_MPLS: RteFlowActionType = 27;
#[doc = " Encapsulate flow in VXLAN tunnel as defined in"]
#[doc = " rte_flow_action_vxlan_encap action structure."]
#[doc = ""]
#[doc = " See struct rte_flow_action_vxlan_encap."]
pub const RTE_FLOW_ACTION_TYPE_VXLAN_ENCAP: RteFlowActionType = 28;
#[doc = " Decapsulate outer most VXLAN tunnel from matched flow."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid VXLAN tunnel (as specified by"]
#[doc = " RFC7348) then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION"]
#[doc = " error."]
pub const RTE_FLOW_ACTION_TYPE_VXLAN_DECAP: RteFlowActionType = 29;
#[doc = " Encapsulate flow in NVGRE tunnel defined in the"]
#[doc = " rte_flow_action_nvgre_encap action structure."]
#[doc = ""]
#[doc = " See struct rte_flow_action_nvgre_encap."]
pub const RTE_FLOW_ACTION_TYPE_NVGRE_ENCAP: RteFlowActionType = 30;
#[doc = " Decapsulate outer most NVGRE tunnel from matched flow."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid NVGRE tunnel (as specified by"]
#[doc = " RFC7637) then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION"]
#[doc = " error."]
pub const RTE_FLOW_ACTION_TYPE_NVGRE_DECAP: RteFlowActionType = 31;
#[doc = " Add outer header whose template is provided in its data buffer"]
#[doc = ""]
#[doc = " See struct rte_flow_action_raw_encap."]
pub const RTE_FLOW_ACTION_TYPE_RAW_ENCAP: RteFlowActionType = 32;
#[doc = " Remove outer header whose template is provided in its data buffer."]
#[doc = ""]
#[doc = " See struct rte_flow_action_raw_decap"]
pub const RTE_FLOW_ACTION_TYPE_RAW_DECAP: RteFlowActionType = 33;
#[doc = " Modify IPv4 source address in the outermost IPv4 header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_IPV4,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_ipv4."]
pub const RTE_FLOW_ACTION_TYPE_SET_IPV4_SRC: RteFlowActionType = 34;
#[doc = " Modify IPv4 destination address in the outermost IPv4 header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_IPV4,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_ipv4."]
pub const RTE_FLOW_ACTION_TYPE_SET_IPV4_DST: RteFlowActionType = 35;
#[doc = " Modify IPv6 source address in the outermost IPv6 header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_IPV6,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_ipv6."]
pub const RTE_FLOW_ACTION_TYPE_SET_IPV6_SRC: RteFlowActionType = 36;
#[doc = " Modify IPv6 destination address in the outermost IPv6 header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_IPV6,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_ipv6."]
pub const RTE_FLOW_ACTION_TYPE_SET_IPV6_DST: RteFlowActionType = 37;
#[doc = " Modify source port number in the outermost TCP/UDP header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_TCP"]
#[doc = " or RTE_FLOW_ITEM_TYPE_UDP, then the PMD should return a"]
#[doc = " RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_tp."]
pub const RTE_FLOW_ACTION_TYPE_SET_TP_SRC: RteFlowActionType = 38;
#[doc = " Modify destination port number in the outermost TCP/UDP header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_TCP"]
#[doc = " or RTE_FLOW_ITEM_TYPE_UDP, then the PMD should return a"]
#[doc = " RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_tp."]
pub const RTE_FLOW_ACTION_TYPE_SET_TP_DST: RteFlowActionType = 39;
#[doc = " Swap the source and destination MAC addresses in the outermost"]
#[doc = " Ethernet header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_ETH,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_MAC_SWAP: RteFlowActionType = 40;
#[doc = " Decrease TTL value directly"]
#[doc = ""]
#[doc = " No associated configuration structure."]
pub const RTE_FLOW_ACTION_TYPE_DEC_TTL: RteFlowActionType = 41;
#[doc = " Set TTL value"]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_ttl"]
pub const RTE_FLOW_ACTION_TYPE_SET_TTL: RteFlowActionType = 42;
#[doc = " Set source MAC address from matched flow."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_ETH,"]
#[doc = " the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_mac."]
pub const RTE_FLOW_ACTION_TYPE_SET_MAC_SRC: RteFlowActionType = 43;
#[doc = " Set destination MAC address from matched flow."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_ETH,"]
#[doc = " the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_mac."]
pub const RTE_FLOW_ACTION_TYPE_SET_MAC_DST: RteFlowActionType = 44;
#[doc = " Increase sequence number in the outermost TCP header."]
#[doc = ""]
#[doc = " Action configuration specifies the value to increase"]
#[doc = " TCP sequence number as a big-endian 32 bit integer."]
#[doc = ""]
#[doc = " @p conf type:"]
#[doc = " @code rte_be32_t * @endcode"]
#[doc = ""]
#[doc = " Using this action on non-matching traffic will result in"]
#[doc = " undefined behavior."]
pub const RTE_FLOW_ACTION_TYPE_INC_TCP_SEQ: RteFlowActionType = 45;
#[doc = " Decrease sequence number in the outermost TCP header."]
#[doc = ""]
#[doc = " Action configuration specifies the value to decrease"]
#[doc = " TCP sequence number as a big-endian 32 bit integer."]
#[doc = ""]
#[doc = " @p conf type:"]
#[doc = " @code rte_be32_t * @endcode"]
#[doc = ""]
#[doc = " Using this action on non-matching traffic will result in"]
#[doc = " undefined behavior."]
pub const RTE_FLOW_ACTION_TYPE_DEC_TCP_SEQ: RteFlowActionType = 46;
#[doc = " Increase acknowledgment number in the outermost TCP header."]
#[doc = ""]
#[doc = " Action configuration specifies the value to increase"]
#[doc = " TCP acknowledgment number as a big-endian 32 bit integer."]
#[doc = ""]
#[doc = " @p conf type:"]
#[doc = " @code rte_be32_t * @endcode"]
#[doc = ""]
#[doc = " Using this action on non-matching traffic will result in"]
#[doc = " undefined behavior."]
pub const RTE_FLOW_ACTION_TYPE_INC_TCP_ACK: RteFlowActionType = 47;
#[doc = " Decrease acknowledgment number in the outermost TCP header."]
#[doc = ""]
#[doc = " Action configuration specifies the value to decrease"]
#[doc = " TCP acknowledgment number as a big-endian 32 bit integer."]
#[doc = ""]
#[doc = " @p conf type:"]
#[doc = " @code rte_be32_t * @endcode"]
#[doc = ""]
#[doc = " Using this action on non-matching traffic will result in"]
#[doc = " undefined behavior."]
pub const RTE_FLOW_ACTION_TYPE_DEC_TCP_ACK: RteFlowActionType = 48;
#[doc = " Set Tag."]
#[doc = ""]
#[doc = " Tag is for internal flow usage only and"]
#[doc = " is not delivered to the application."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_tag."]
pub const RTE_FLOW_ACTION_TYPE_SET_TAG: RteFlowActionType = 49;
#[doc = " Set metadata on ingress or egress path."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_meta."]
pub const RTE_FLOW_ACTION_TYPE_SET_META: RteFlowActionType = 50;
#[doc = " Modify IPv4 DSCP in the outermost IP header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_IPV4,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_dscp."]
pub const RTE_FLOW_ACTION_TYPE_SET_IPV4_DSCP: RteFlowActionType = 51;
#[doc = " Modify IPv6 DSCP in the outermost IP header."]
#[doc = ""]
#[doc = " If flow pattern does not define a valid RTE_FLOW_ITEM_TYPE_IPV6,"]
#[doc = " then the PMD should return a RTE_FLOW_ERROR_TYPE_ACTION error."]
#[doc = ""]
#[doc = " See struct rte_flow_action_set_dscp."]
pub const RTE_FLOW_ACTION_TYPE_SET_IPV6_DSCP: RteFlowActionType = 52;
#[doc = " Report as aged flow if timeout passed without any matching on the"]
#[doc = " flow."]
#[doc = ""]
#[doc = " See struct rte_flow_action_age."]
#[doc = " See function rte_flow_get_aged_flows"]
#[doc = " see enum RTE_ETH_EVENT_FLOW_AGED"]
#[doc = " See struct rte_flow_query_age"]
pub const RTE_FLOW_ACTION_TYPE_AGE: RteFlowActionType = 53;
#[doc = " The matching packets will be duplicated with specified ratio and"]
#[doc = " applied with own set of actions with a fate action."]
#[doc = ""]
#[doc = " See struct rte_flow_action_sample."]
pub const RTE_FLOW_ACTION_TYPE_SAMPLE: RteFlowActionType = 54;
#[doc = " Describe action shared across multiple flow rules."]
#[doc = ""]
#[doc = " Allow multiple rules reference the same action by handle (see"]
#[doc = " struct rte_flow_shared_action)."]
pub const RTE_FLOW_ACTION_TYPE_SHARED: RteFlowActionType = 55;

pub type RteFlowActionType = dpdk_sys::rte_flow_action_type;

#[derive(Debug)]
pub struct RteFlowAttr {
    pub data: dpdk_sys::rte_flow_attr,
}

impl RteFlowAttr {
    pub fn new() -> Self {
        return RteFlowAttr{
            data: unsafe {
                std::mem::zeroed()
            },
        }
    }

    pub fn set_ingress(&mut self, ingress: u32) {
        self.data.set_ingress(ingress)
    }

    pub fn set_egress(&mut self, egress: u32) {
        self.data.set_egress(egress)
    }

    pub fn set_transfer(&mut self, transfer: u32) {
        self.data.set_transfer(transfer)
    }

    pub fn set_group(&mut self, group: u32) {
        self.data.group = group
    }

    pub fn set_priority(&mut self, prio: u32) {
        self.data.priority = prio
    }
}

#[derive(Debug)]
pub struct RteFlowItem {
    pub data: dpdk_sys::rte_flow_item,
}

impl RteFlowItem {
    pub fn new() -> Self{
        return RteFlowItem{
            data: unsafe {
                std::mem::zeroed()
            },
        }
    }

    pub fn set_item_type(&mut self, item_type: RteFlowItemType) {
        self.data.type_ = item_type;
    }

    pub fn set_item_spec(&mut self, spec: *const ::std::os::raw::c_void) {
        self.data.spec = spec;
    }

    pub fn set_item_mask(&mut self, mask: *const ::std::os::raw::c_void) {
        self.data.mask = mask;
    }
}

#[derive(Debug)]
pub struct  RteFlowAction {
    pub data: dpdk_sys::rte_flow_action,
}

impl RteFlowAction {
    pub fn new() -> Self{
        return RteFlowAction{
            data: unsafe {
                std::mem::zeroed()
            },
        }
    }

    pub fn set_actiion_type(&mut self, action_type: RteFlowActionType) {
        self.data.type_ = action_type;
    }

    pub fn set_action_conf(&mut self, conf: *const ::std::os::raw::c_void ) {
        self.data.conf = conf
    }
}

#[derive(Debug)]
pub struct RteFlowError {
    pub data: *mut dpdk_sys::rte_flow_error,
}

impl RteFlowError {
    pub fn new() -> Self {
        return RteFlowError{
            data: unsafe {
                std::mem::zeroed()
            },
        }
    }
}

#[derive(Debug)]
pub struct RteFlow {
    pub data: *mut dpdk_sys::rte_flow,
}

impl RteFlow {
    pub fn new() -> *mut Self {
        return &mut RteFlow{
            data: unsafe {
                std::mem::zeroed()
            },
        }
    }
}

pub unsafe fn isolate_port(port_id: u16, set: i32) {
    let mut err:RteFlowError = RteFlowError::new();
    dpdk_sys::rte_flow_isolate(port_id, set, err.data);
}

pub unsafe fn create_rte_flow(port_id: u16,
    attr: *mut RteFlowAttr,
    pattern: Vec<RteFlowItem>,
    actions: Vec<RteFlowAction>,
    error: *mut RteFlowError,
) ->  *const RteFlow {
    let mut flow_match: Vec<dpdk_sys::rte_flow_item> = Vec::new();
    for p in pattern {
        flow_match.push(p.data);
    }
    let flow = dpdk_sys::rte_flow_create(port_id,&(*attr).data, flow_match.as_ptr(), &actions[0].data, (*error).data);
    let ret = RteFlow::new();
    (*ret).data = flow;
    return ret;
}
