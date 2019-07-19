let rec many = (num, fn) =>
  num == 0 ? [] : [fn(num), ...many(num - 1, fn)];

let pi = 3.1415;

[@react.component]
let make = (~count, ~onChange) => {
  let size = 50;
  let scale = pi *. 2.0 /. float_of_int(count);
  <div className=Styles.row>
    <Tippy content="Rotational symmetry">
      <div className=Css.(style([position(`relative)]))>
        <svg width={string_of_int(size)} height={string_of_int(size)}>
          {many(count, i =>
             <path
               stroke="black"
               strokeWidth="2px"
               d={
                   let c = float_of_int(size) /. 2.0;
                   let r0 = 15.0;
                   let r1 = 25.0;
                   let a0 =
                     float_of_int(i) *. scale -. pi /. 2.0 -. pi /. 10.0;
                   let a1 = a0 +. pi /. 10.0;
                   Printf.sprintf(
                     "M%0.2f,%0.2f L%0.2f,%0.2f",
                     c +. cos(a0) *. r0,
                     c +. sin(a0) *. r0,
                     c +. cos(a1) *. r1,
                     c +. sin(a1) *. r1,
                   );
                 }
             />
           )
           ->Array.of_list
           ->React.array}
        </svg>
        <div
          className=Css.(
            style([
              position(`absolute),
              top(`zero),
              bottom(`zero),
              left(`zero),
              right(`zero),
              display(`flex),
              justifyContent(`center),
              alignItems(`center),
            ])
          )>
          <input
            value={string_of_int(count)}
            type_="number"
            className=Css.(
              style([
                marginLeft(px(15)),
                backgroundColor(`transparent),
                width(px(40)),
                textAlign(`center),
                borderStyle(`none),
              ])
            )
            min=1
            onChange={evt =>
              onChange(int_of_string(evt->ReactEvent.Form.target##value))
            }
          />
        </div>
      </div>
    </Tippy>
    <div className=Css.(style([flexBasis(px(8)), flexShrink(0)])) />
    <div className=Styles.column>
      <button onClick={_ => onChange(count + 1)}>
        {React.string("+")}
      </button>
      {Styles.spacer(8)}
      <button onClick={_ => onChange(max(1, count - 1))}>
        {React.string("-")}
      </button>
    </div>
  </div>;
};