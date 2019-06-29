
module Async = {
  let let_ = (v, fn) => Js.Promise.then_(fn, v);
  let resolve = Js.Promise.resolve;
  module Consume = {
    let let_ = (v, fn) => {
      Js.Promise.then_(
        v => {
          let () = fn(v);
          Js.Promise.resolve();
        },
        v,
      )
      ->ignore;
    };
  };
};

module Opt = {
  let force = m =>
    switch (m) {
    | None => failwith("unwrapping option")
    | Some(m) => m
    };
  module Consume = {
    let let_ = (v, fn) =>
      switch (v) {
      | None => ()
      | Some(m) => fn(m)
      };
  };
};
