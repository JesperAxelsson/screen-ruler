
// use parser::Parser;

use crate::parser::Parser;

// screen 0: minimum 8 x 8, current 5120 x 1600, maximum 32767 x 32767
#[derive(Debug, Eq, PartialEq)]
pub struct Screen {
    pub number: u64,
    pub minimum: (u64, u64),
    pub current: (u64, u64),
    pub maximum: (u64, u64),
}

fn parse_screen(parser: &mut Parser) -> Option<Screen> {
    parser
        .parse_token("Screen")
        .or(parser.parse_token("screen"))?;
    parser.skip_whitespace();
    let number = parser.parse_u64()?;

    let minimum = parser.find(&parse_resolution)?;
    let current = parser.find(&parse_resolution)?;
    let maximum = parser.find(&parse_resolution)?;

    parser.skip_eol();

    return Some(Screen {
        number,
        minimum,
        current,
        maximum,
    });
}

// 2560 x 1440
fn parse_resolution(parser: &mut Parser) -> Option<(u64, u64)> {
    let x = parser.parse_u64()?;
    parser.skip_whitespace();
    parser.parse_token("x")?;
    parser.skip_whitespace();
    let y = parser.parse_u64()?;

    return Some((x, y));
}

// DVI-D-0 connected 2560x1440+2560+0 (normal left inverted right x axis y axis) 597mm x 336mm
// DVI-D-0 connected (normal left inverted right x axis y axis)
// DP-0 disconnected (normal left inverted right x axis y axis)
// DP-4 connected primary 2560x1600+0+0 (normal left inverted right x axis y axis) 641mm x 401mm

fn parse_connection(parser: &mut Parser) -> Option<Port> {
    let port = parser.parse_word1()?;
    parser.skip_whitespace();

    let connected = if let Some(con) = parser.parse_word1() {
        con == "connected"
    } else {
        false
    };

    parser.skip_whitespace();
    let primary = parser.parse_token("primary");
    parser.skip_whitespace();
    let resolution = parse_resolution(parser);

    parser.skip_eol();

    return Some(Port {
        port,
        connected,
        primary: primary.is_some(),
        resolution,
    });
}

pub fn parse_output(data: String) -> Vec<Port> {
    let mut parser = Parser::new(data);
    let mut result = Vec::new();

    while !parser.eof() {
        if let Some(port) = parse_connection(&mut parser) {
            result.push(port);
        } else {
            parser.skip_eol();
        }
    }

    return result;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Port {
    pub port: String,
    pub connected: bool,
    pub primary: bool,
    pub resolution: Option<(u64, u64)>,
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_resolution() {
        let mut parser = Parser::new("12x67".to_owned());
        assert_eq!(parse_resolution(&mut parser), Some((12, 67)));
    }

    #[test]
    fn test_parse_resolution_white_space() {
        let mut parser = Parser::new("12 x 67".to_owned());
        assert_eq!(parse_resolution(&mut parser), Some((12, 67)));
    }

    #[test]
    fn test_parse_screen() {
        let mut parser = Parser::new(
            "screen 0: minimum 8 x 8, current 5120 x 1600, maximum 32767 x 32767".to_owned(),
        );
        assert_eq!(
            parse_screen(&mut parser),
            Some(Screen {
                number: 0,
                minimum: (8, 8),
                current: (5120, 1600),
                maximum: (32767, 32767),
            })
        );
    }

    // DP-0 disconnected (normal left inverted right x axis y axis)
    // DP-4 connected primary 2560x1600+0+0 (normal left inverted right x axis y axis) 641mm x 401mm

    #[test]
    fn test_parse_port_dvi_connected() {
        let mut parser = Parser::new(
        "DVI-D-0 connected 2560x1440+2560+0 (normal left inverted right x axis y axis) 597mm x 336mm".to_owned(),
        );
        assert_eq!(
            parse_connection(&mut parser),
            Some(Port {
                port: "DVI-D-0".to_string(),
                connected: true,
                primary: false,
                resolution: Some((2560, 1440)),
            })
        );
    }

    #[test]
    fn test_parse_port_dvi_connected_not_active() {
        let mut parser =
            Parser::new("DVI-D-0 connected (normal left inverted right x axis y axis)".to_owned());
        assert_eq!(
            parse_connection(&mut parser),
            Some(Port {
                port: "DVI-D-0".to_string(),
                connected: true,
                primary: false,
                resolution: None,
            })
        );
    }

    #[test]
    fn test_parse_port_dp_disconnected() {
        let mut parser =
            Parser::new("DP-0 disconnected (normal left inverted right x axis y axis)".to_owned());
        assert_eq!(
            parse_connection(&mut parser),
            Some(Port {
                port: "DP-0".to_string(),
                connected: false,
                primary: false,
                resolution: None,
            })
        );
    }

    #[test]
    fn test_parse_port_dp_connected_primary() {
        let mut parser =
            Parser::new("DP-4 connected primary 2560x1600+0+0 (normal left inverted right x axis y axis) 641mm x 401mm".to_owned());
        assert_eq!(
            parse_connection(&mut parser),
            Some(Port {
                port: "DP-4".to_string(),
                connected: true,
                primary: true,
                resolution: Some((2560, 1600)),
            })
        );
    }
}
