#[cfg(test)]
use crate::parse_iban;

#[test]
fn test_a_lot_of_different_ibans() {
    let ibans: Vec<&str> = vec![
        "AL35202111090000000001234567",
        "AD1400080001001234567890",
        "AZ96AZEJ00000000001234567890",
        "BH02CITI00001077181611",
        "BE71096123456769",
        "BA393385804800211234",
        "BR1500000000000010932840814P2",
        "BG18RZBB91550123456789",
        "CR23015108410026012345",
        "HR1723600001101234565",
        "CY21002001950000357001234567",
        "DK9520000123456789",
        "DO22ACAU00000000000123456789",
        "DE91100000000123456789",
        "EG800002000156789012345180002",
        "SV43ACAT00000000000000123123",
        "EE471000001020145685",
        "FO9264600123456789",
        "FI1410093000123458",
        "FR7630006000011234567890189",
        "GE60NB0000000123456789",
        "GI04BARC000001234567890",
        "GR9608100010000001234567890",
        "GL8964710123456789",
        "GT20AGRO00000000001234567890",
        "HU93116000060000000012345676",
        "IE64IRCE92050112345678",
        "IS030001121234561234567890",
        "IQ20CBIQ861800101010500",
        "IL170108000000012612345",
        "IT60X0542811101000000123456",
        "JO71CBJO0000000000001234567890",
        "KZ563190000012344567",
        "XK051212012345678906",
        "KW81CBKU0000000000001234560101",
        "LV97HABA0012345678910",
        "LB92000700000000123123456123",
        "LI7408806123456789012",
        "LT601010012345678901",
        "LU120010001234567891",
        "VG21PACG0000000123456789",
        "MK07200002785123453",
        "MT31MALT01100000000000000000123",
        "MR1300020001010000123456753",
        "MU43BOMM0101123456789101000MUR",
        "MD21EX000000000001234567",
        "MC5810096180790123456789085",
        "ME25505000012345678951",
        "NL02ABNA0123456789",
        "NO8330001234567",
        "AT483200000012345864",
        "PK36SCBL0000001123456702",
        "PS92PALS000000000400123456702",
        "PL10105000997603123456789123",
        "PT50002700000001234567833",
        "QA54QNBA000000000000693123456",
        "RO09BCYP0000001234567890",
        "LC14BOSL123456789012345678901234",
        "SM76P0854009812123456789123",
        "ST23000200000289355710148",
        "SA4420000001234567891234",
        "RS35105008123123123173",
        "SI56192001234567892",
        "SK8975000000000012345671",
        "ES7921000813610123456789",
        "TL380010012345678910106",
        "CZ5508000000001234567899",
        "TN5904018104004942712345",
        "TR320010009999901234567890",
        "UA903052992990004149123456789",
        "GB98MIDL07009312345678",
        "AE460090000000123456789",
        "BY86AKBB10100000002966000000",
        "SE7280000810340009783242",
        "CH5604835012345678009",
    ];

    for iban in ibans {
        println!("Parsing {iban}");
        let result = parse_iban(iban);
        assert!(result.is_valid);
    }
}