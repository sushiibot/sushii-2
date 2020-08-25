use lazy_static::lazy_static;
use regex::Regex;
use serenity::framework::standard::Args;

pub fn parse_id_reason(args: Args) -> (Vec<u64>, String) {
    lazy_static! {
        // Can overflow, so need to handle later
        static ref RE: Regex = Regex::new(r"\d{18,19}").unwrap();
    }

    let ids_and_reason = args.rest();

    let (ids, end) = RE
        .find_iter(ids_and_reason)
        .fold((Vec::new(), 0), |mut acc, id_match| {
            if let Ok(id) = id_match.as_str().parse::<u64>() {
                acc.0.push(id);
                acc.1 = id_match.end();
            }

            acc
        });

    let reason = ids_and_reason[end..].trim().to_string();

    (ids, reason)
}

#[test]
fn parses_id_and_reason() {
    use serenity::framework::standard::Delimiter;

    let ids_exp = vec![145764790046818304, 193163974471188480, 151018674793349121];
    let reason_exp = "some reason text";

    let input_strs = vec![
        // Comma separated
        "145764790046818304,193163974471188480,151018674793349121 some reason text",
        // Space separated
        "145764790046818304 193163974471188480 151018674793349121 some reason text",
        // Random chars in middle
        "145764790046818304   193163974471188480 aoweifjf 151018674793349121 some reason text",
    ];

    for s in input_strs {
        let args = Args::new(s, &[Delimiter::Single(' ')]);

        let (ids, reason) = parse_id_reason(args);

        assert_eq!(ids, ids_exp);
        assert_eq!(reason, reason_exp);
    }
}
