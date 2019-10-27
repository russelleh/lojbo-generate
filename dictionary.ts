(function () {
  document.addEventListener("DOMContentLoaded", function (e) {
    let words = // $(cargo run | jq -csM)
    document.getElementsByTagName("body")[0].innerHTML = words.map((word) => {
      const match   = word.meaning.match(/x[1-5]/g);
      return (() => { if (match) {
        return "<div class='root_word'>" +
          "<div class='word'>" +
            "<h4 class='value'>" +
              word.value +
            "</h4>" +
            "<ul class='sources'>" +
              word.sources.sort((source_a, source_b) => {
                if (source_a.language > source_b.language) {
                  return 1
                } else if (source_a.language < source_b.language) {
                  return -1
                } else {
                  return 0
                }
              }).filter((source) => {
                return source.source !== ""
              }).filter((source, index, sources) => {
                return sources.map((source) => {
                  return source.source
                }).indexOf(source.source) === index
              }).map((source) => {
                return "<li class='source " + source.language + "'>" +
                  source.source.replace(/[\[\]]/g, "")
                "</li>"
              }).join("") +
            "</ul>" +
          "</div>" +
          "<div class='meaning'>" +
            match.reduce((s, x) => {
              const n = x.substr(1)
              return s.replace(
                / \/\/.*/, ""
              ).replace(
                /\(.*\)/, ""
              ).replace(
                x, "x<sub>" + n + "</sub>"
              )
            }, word.meaning) +
          "</div>" +
        "</div>"
      } else {
        return ""
      } })();
    }).join("")
  })
})()
