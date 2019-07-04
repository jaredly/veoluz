open Css;

let title =
  style([
    fontSize(px(12)),
    fontWeight(`num(600)),
    color(Colors.accent),
    marginBottom(px(8)),
  ]);

let control =
  style([
    borderRadius(px(4)),
    boxShadow(~spread=px(2), ~blur=px(5), Colors.accent),
    // border(px(2), `solid, Colors.accent),
    backgroundColor(Colors.control),
    padding2(~v=px(6), ~h=px(8)),
    marginBottom(px(8)),
  ]);
