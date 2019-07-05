open Css;

let title =
  style([
    fontSize(px(12)),
    fontWeight(`num(600)),
    color(Colors.accent),
    marginBottom(px(8)),
  ]);

let titleNoMargin =
  style([
    fontSize(px(12)),
    fontWeight(`num(600)),
    color(Colors.accent),
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

let spacer = v => <div style=ReactDOMRe.Style.make(~flexBasis=string_of_int(v) ++ "px", ()) />;

let row = style([display(`flex), flexDirection(`row), alignItems(`center)])

let flatButton = textColor => style([
  backgroundColor(`transparent),
  color(textColor)
]);
