(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *)

open Base
open OUnit2
module Deferred = Server.ServerState.DeferredUpdateEvents

let change_event path =
  PyrePath.create_absolute path
  |> SourcePath.create
  |> SourcePath.Event.(create ~kind:Kind.CreatedOrChanged)


let remove_event path =
  PyrePath.create_absolute path |> SourcePath.create |> SourcePath.Event.(create ~kind:Kind.Deleted)


let assert_events ~context ~expected actual =
  assert_equal
    ~ctxt:context
    ~cmp:[%compare.equal: SourcePath.Event.t list]
    ~printer:(fun events -> [%sexp_of: SourcePath.Event.t list] events |> Sexp.to_string_hum)
    expected
    actual


let test_deferred_updates_empty context =
  assert_events (Deferred.create () |> Deferred.get_all) ~context ~expected:[]


let test_deferred_updates_add context =
  let deferred = Deferred.create () in
  let event_a = change_event "/foo/a.py" in
  let event_b = remove_event "/foo/b.py" in
  let event_c = remove_event "/bar/c.py" in
  let event_d = change_event "/bar/d.py" in
  Deferred.add deferred [event_a; event_b];
  Deferred.add deferred [event_c; event_d];
  assert_events (Deferred.get_all deferred) ~context ~expected:[event_a; event_b; event_c; event_d];
  ()


let test_deferred_updates_clear context =
  let deferred = Deferred.create () in
  Deferred.add deferred [change_event "/foo/a.py"; remove_event "/foo/b.py"];
  Deferred.clear deferred;
  assert_events (Deferred.get_all deferred) ~context ~expected:[];
  ()


let () =
  "state"
  >::: [
         "deferred_updates_empty" >:: test_deferred_updates_empty;
         "deferred_updates_add" >:: test_deferred_updates_add;
         "deferred_updates_clear" >:: test_deferred_updates_clear;
       ]
  |> Test.run