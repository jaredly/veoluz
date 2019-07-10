open Css;

module Text = {
  let small = 16;
  let normal = 20;
  let large = 24;
};

module Spacing = {
  let small = 8;
  let medium = 16;
  let large = 24;
};

let title =
  style([
    fontSize(px(Text.small)),
    fontWeight(`num(600)),
    color(Colors.accent),
    marginBottom(px(8)),
  ]);

let titleNoMargin =
  style([
    fontSize(px(Text.small)),
    fontWeight(`num(600)),
    color(Colors.accent),
  ]);

let join = styles => String.concat(" ", styles);

let control =
  style([
    borderRadius(px(4)),
    boxShadow(~spread=px(2), ~blur=px(5), Colors.accent),
    // border(px(2), `solid, Colors.accent),
    backgroundColor(Colors.control),
    padding2(~v=px(6), ~h=px(8)),
    marginBottom(px(8)),
  ]);

let column = style([display(`flex), flexDirection(`column)]);

let controlColumn = join([control, column]);

let spacer = v =>
  <div
    style={ReactDOMRe.Style.make(~flexBasis=string_of_int(v) ++ "px", ())}
  />;
let fullSpace = <div style={ReactDOMRe.Style.make(~flex="1", ())} />;

let row =
  style([display(`flex), flexDirection(`row), alignItems(`center)]);

let flatButton = textColor =>
  style([backgroundColor(`transparent), color(textColor)]);

let iconButton = active =>
  style([
    padding(px(4)),
    backgroundColor(active ? Colors.accent : Colors.button),
    hover(active ? [backgroundColor(Colors.accent)] : []),
  ]);

let multiButton =
  style([
    padding(px(4)),
    backgroundColor(`transparent),
    disabled([
      backgroundColor(Colors.accent),
      hover([backgroundColor(Colors.accent)]),
    ]),
  ]);