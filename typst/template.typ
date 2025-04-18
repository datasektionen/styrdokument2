#let project(
  date: none,
  title: "",
  body,
) = {
  let cerise = rgb("#e83d84")
  let datagray = rgb("#888888")
  set text(lang: "sv")

  set text(
    font: "Lato",
    size: 11pt,
  )

  // How text was meant to be
  set par(justify: true)

  show heading: it => {
    set text(
      size:
        if it.level == 1
            {22pt}
        else if it.level == 2
          {18pt}
        else if it.level == 3
          {14pt}
        else {12pt},

      fill: if it.level == 4 {black} else {cerise},
      style: if it.level == 4 {"italic"} else {"normal"},
    )
    [#it]
  }

  // Cover page
  align(center + horizon, [
    #image(
      "figures/formal_logo.svg",
      width: 30em
    )
    = *#title*
    == Konglig Datasektionen
  ])
  pagebreak()

  set page(
    margin: (top: 3.75cm, bottom: 3cm),
    header-ascent: 6mm,
    header: [
      #set text(size: 10pt)
      #grid(
        columns: (1fr, 5cm, 1fr),
        // date -> gray
        align(left + horizon, [#title \ #set text(datagray); #date]),
        align(center + horizon, image(
          "figures/shield.svg",
          width: 1.5cm,
        )),
        align(right + horizon, [#set text(datagray); #title \ #context counter(page).display("1/1", both: true)])
      )
    ],
  
    footer: [
      #align(center, [
        #set text(size: 10pt, datagray)
        Konglig Datasektionen, THS 100 44, datasektionen.se
        ]
      )
    ],
  )

  body
}
