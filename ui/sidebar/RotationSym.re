let rec many = (num, fn) =>
  num == 0 ? [] : [fn(num), ...many(num - 1, fn)];

let pi = 3.1415;

[@react.component]
let make = (~count, ~onChange) => {
  let size = 100;
  let scale = pi *. 2.0 /. float_of_int(count);
  <svg width={string_of_int(size)} height={string_of_int(size)}>
    {many(count, i =>
       <path
         stroke="black"
         strokeWidth="2px"
         d={
             let c = float_of_int(size) /. 2.0;
             let r0 = 25.0;
             let r1 = 40.0;
             let a0 = float_of_int(i) *. scale -. pi /. 2.0 -. pi /. 10.0;
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
  </svg>;
};