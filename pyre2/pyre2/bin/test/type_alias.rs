/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::simple_test;

simple_test!(
    test_type_alias_simple,
    r#"
from typing import assert_type
type X = int
def f(x: X):
    assert_type(x, int)
    "#,
);

simple_test!(
    test_type_alias_generic,
    r#"
from typing import assert_type
type X[T] = list[T]
def f(x: X[int]):
    assert_type(x, list[int])
    "#,
);

simple_test!(
    test_type_alias_missing_quantifieds,
    r#"
from typing import TypeVar
T = TypeVar('T')
type X = list[T]  # E: Type parameters used in `X`
    "#,
);

simple_test!(
    test_type_alias_unused_quantifieds,
    r#"
# Questionable code, but not an error
type X[T] = list
    "#,
);

simple_test!(
    test_bad_type_alias,
    r#"
type X = 1  # E: Expected `X` to be a type alias, got Literal[1]
    "#,
);

simple_test!(
    test_generic_alias_implicit,
    r#"
from typing import TypeVar, assert_type
T = TypeVar('T')
X = list[T]
def f(x: X[int]):
    assert_type(x, list[int])
    "#,
);

simple_test!(
    test_generic_alias_explicit,
    r#"
from typing import TypeAlias, TypeVar, assert_type
T = TypeVar('T')
X: TypeAlias = list[T]
def f(x: X[int]):
    assert_type(x, list[int])
    "#,
);

simple_test!(
    test_generic_alias_union,
    r#"
from typing import TypeVar, assert_type
T = TypeVar('T')
X = T | list[T]
def f(x: X[int]):
    assert_type(x, int | list[int])
    "#,
);

simple_test!(
    test_generic_alias_callable,
    r#"
from typing import Callable, TypeVar, assert_type
T = TypeVar('T')
X1 = Callable[..., T]
X2 = Callable[[T], str]
def f(x1: X1[int], x2: X2[int]):
    assert_type(x1, Callable[..., int])
    assert_type(x2, Callable[[int], str])
    "#,
);

simple_test!(
    test_generic_alias_annotated,
    r#"
from typing import Annotated, TypeVar, assert_type
T = TypeVar('T')
X = Annotated[T, 'the world is quiet here']
def f(x: X[int]):
    assert_type(x, int)
    "#,
);

simple_test!(
    test_bad_annotated_alias,
    r#"
from typing import TypeAlias
X: TypeAlias = 1  # E: Expected `X` to be a type alias, got Literal[1]
    "#,
);

simple_test!(
    test_attribute_access,
    r#"
from typing import TypeAlias
X1 = int
X2: TypeAlias = int
type X3 = int

X1.__add__  # ok
X2.__add__  # ok
X3.__add__  # E: Cannot use type alias `X3`
    "#,
);