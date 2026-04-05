plugin add nu_plugin_query
plugin use query

const USER = "yuikisaito"
const FROM_SECOND_STATE = "last_from_second.txt"
const SLEEP_SEC = 3sec

def main [] {
  mut from_second = 0
  if ($FROM_SECOND_STATE | path exists) {
    $from_second = open $FROM_SECOND_STATE
  }

  let subs = http get $"https://kenkoooo.com/atcoder/atcoder-api/v3/user/submissions?user=($USER)&from_second=($from_second)"

  for $sub in $subs {
    let sub_id = $sub.id
    let contest_id = $sub.contest_id
    let problem_id = $sub.problem_id
    let lang = $sub.language
    let ext = get_extension $lang

    let sub_dir = $"submissions/($contest_id)/($problem_id)"
    mkdir $sub_dir

    let code_file = $"($sub_dir)/($sub_id).($ext)"

    if ($code_file | path exists) {
      continue
    }

    let sub_html = http get $"https://atcoder.jp/contests/($contest_id)/submissions/($sub_id)"
    let code = $sub_html | query web --query "#submission-code" | flatten | get 0
    let status = $sub_html | query web --query "#judge-status > span" | flatten | get 0

    $code | save $code_file
    git add $code_file
    GIT_AUTHOR_DATE=$sub_id git commit -m $"submit #($sub_id) [($status)]"

    sleep $SLEEP_SEC
  }

  date now | format date "%s" | save -f $FROM_SECOND_STATE

  git push
}

def get_extension [language: string] {
  match ($language | str downcase | str replace -r "\\s?\\(.*\\)" "") {
    "rust" => "rs"
    "python" => "py"
    "c++ 23" => "cpp"
    _ => "txt"
  }
}
