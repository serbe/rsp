use crate::netutils::crawl;

use regex::Regex;

pub async fn get() -> Result<Vec<String>, RspError> {
    let mut ips = Vec::new();
    for link in cybersyndromenet_links() {
        if let Ok(body) = crawl(&link) {
            for ip in cybersyndromenet_ips(&body) {
                ips.push(ip);
            }
        }
    }
    Ok(ips)
}

fn cybersyndromenet_links() -> Vec<String> {
    let links = vec![
        "http://www.cybersyndrome.net/pla6.html".to_string(),
        "http://www.cybersyndrome.net/pld6.html".to_string(),
    ];
    links
}

fn cybersyndromenet_ips(body: &str) -> Vec<String> {
    let mut ips = Vec::new();

    let re_as = Regex::new(r"var as=\[([\d,]+?)\]").unwrap();
    let re_ps = Regex::new(r"var ps=\[([\d,]+?)\]").unwrap();
    let re_n = Regex::new(r"var n=(\(.+?);").unwrap();

    if re_as.is_match(body) && re_ps.is_match(body) && re_n.is_match(body) {
        let s_as = re_as.find(body).unwrap().as_str().split(",");
        // dbg!(s_as);
        let s_ps = re_ps
            .captures(body)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(",");
        // dbg!(s_ps);
        let n = re_n.captures(body).unwrap().get(1).unwrap().as_str();
        // dbg!(n);

        let re_ps_num = Regex::new(r"(ps\[\d+\])").unwrap();
        // let f_ps = re_ps_num.
    }

    // as := strings.Split(string(reAS.FindSubmatch(body)[1]), ",")
    // ps := strings.Split(string(rePS.FindSubmatch(body)[1]), ",")
    // n := string(reN.FindSubmatch(body)[1])
    // rePSNum := regexp.MustCompile(`(ps\[\d+\])`)
    // fPS := rePSNum.FindAllString(n, -1)
    // for _, item := range fPS {
    // 	n = strings.Replace(n, item, ps2s(ps, item), 1)
    // }
    // res, _ := strconv.Atoi(calc(n))
    // as = rotate(as, res)
    // for i, port := range ps {
    // 	ips = append(ips, "http://"+as[i*4]+"."+as[i*4+1]+"."+as[i*4+2]+"."+as[i*4+3]+":"+port)
    // }
    ips
}
