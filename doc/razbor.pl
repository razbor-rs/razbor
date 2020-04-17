:- use_module(library(pprint)).
:- use_module(library(yall)).

:- expects_dialect(sicstus).
:- [tox].

:- op(1100, xfy, user:(//)).
:- op(990, xfy, user:(→)).

L // R :- if(L, true, R).

match(X, [V|Cases]) :-
    ( V = (Val → Goal), X = Val *->
        call(Goal)
    ; X = V *->
        true
    ;
        match(X, Cases)
    ).

% def path: type.
% def path: list string -> list string -> path.

% def endinanness: type.
% def big_endian: endinanness.
% def little_endian: endinanness.

% def rel: type.
% def rexpr: type.

% def hole: rexpr.
% def sizeof: rexpr.
% def rint: int -> rexpr.
% def ref: path -> rexpr.
% def size: list rel -> rexpr.

% def (<): rexpr -> rexpr -> rel.
% def (≤): rexpr -> rexpr -> rel.
% def (=): rexpr -> rexpr -> rel.
% def (≥): rexpr -> rexpr -> rel.
% def (>): rexpr -> rexpr -> rel.

% def value: type.
% def vbool: bool -> value.
% def vint: int -> value.

% def ty: type.
% def top: ty.
% def bottom: ty.

% def boolean: ty.
% def integer: ty.

% def u: int -> endinanness -> ty.
% def s: int -> endinanness -> ty.

% def arr: ty -> list rel -> ty.
% def str: list rel -> ty.

% def meet: list ty -> ty.
% def join: list ty -> ty.

% def ref: path -> ty.

% def (⋮): ty -> list rel -> ty.
% def (∈): value -> ty -> ty.

% def prod: list path -> ty.

% def reduced: ty -> ty.

% def (<:): path -> ty -> o.
% def ctypeof: path -> ty -> o.

% def simpl: ty -> ty -> o.
% def subst: ty -> ty -> o.
% def reduce_meet: ty -> ty -> ty -> o.
% def reduce_join: list ty -> list ty -> o.

as_path(S, path(Mods, Data)) :-
    split_string(S, " ", "", [ModsStr, DataStr]) *->
        split_string(ModsStr, ".", "", Mods),
        split_string(DataStr, ".", "", Data)
    ;
        split_string(S, ".", "", Mods),
        Data = [].

ctypeof(P, T) :-
    P <: Tx,
    subst(Tx, Ty),
    simpl(Ty, T).

print_tox_ctypeof :-
    findall(P, P <: _, RawTypes),
    maplist(ctypeof_pair, RawTypes, CTypes),
    WriteOps = [spacing(next_argument)],
    print_term(CTypes, [write_options(WriteOps)]).

ctypeof_pair(P, P <: T) :- ctypeof(P, T).

print_ctypeof(PathStr) :-
    as_path(PathStr, Path),
    ctypeof(Path, T),
    WriteOps = [
        right_margin(60),
        write_options([
            spacing(next_argument)
        ])
    ],
    print_term(T, WriteOps).

is_simple(E) :-
    E \= prod(_),
    E \= join(_).

subst_(meet([X|Xs]), meet([Y|Ys])) :-
    subst(X, Y),
    subst(meet(Xs), meet(Ys)).
subst_(Tx ⋮ P, Ty ⋮ P) :-
    subst(Tx, Ty).
subst_(V ∈ Tx, V ∈ Ty) :-
    subst(Tx, Ty).
subst_(ref(P), T) :-
    ctypeof(P, Tx),
    if(is_simple(Tx), T = (Tx ⋅ [P]), T = ref(P)).
subst(Tx, Ty) :-
    subst_(Tx, Ty) // Tx = Ty.

simpl_(Tx ⋮ P, Ty ⋮ P) :-
    simpl(Tx, Ty).
simpl_(V ∈ Tx, V ∈ Ty) :-
    simpl(Tx, Ty).
simpl_(meet([]), top).
simpl_(meet([T]), T).
simpl_(meet([X|Xs]), T) :-
    reduce_meet(X, meet(Xs), M),
    simpl(M, T).
simpl_(join([X|[]]), Y) :-
    simpl(X, Y).
simpl_(join([X|Xs]), join(Z)) :-
    simpl(X, Y),
    simpl(join(Xs), join(Ys)),
    reduce_join([Y|Ys], Z).
simpl(Tx, Ty) :-
    simpl_(Tx, Ty) // Tx = Ty.

reduce_join(join([X|Xs]), Y) :-
    append(X, Xs, Z),
    reduce_join(Z, Y).
reduce_join([X|Ys], [X|Zs]) :-
    reduce_join(Ys, Zs).
reduce_join([], []).

reduce_meet(L, R, M) :- match((L, R, M), [
    (T, T, T),
    (top, T, T),
    (T, top, T),
    (bottom, _, bottom),
    (integer, u(S, E), u(S, E)),
    (u(S, E), integer, u(S, E)),

    (Tx ⋮ Px, Ty ⋮ Py, T ⋮ P) → (
        reduce_meet(Tx, Ty, T),
        append(Px, Py, P)
    ),
    (Tx ⋮ P, V ∈ Ty, V ∈ (T ⋮ P)) → 
        reduce_meet(Tx, Ty, T),
    (V ∈ Tx, Ty ⋮ P, V ∈ (T ⋮ P)) → 
        reduce_meet(Tx, Ty, T),
    (Tx ⋮ P, Ty, T ⋮ P) →
        reduce_meet(Tx, Ty, T),
    (Tx, Ty ⋮ P, T ⋮ P) →
        reduce_meet(Tx, Ty, T),
    (Vx ∈ Tx, Vy ∈ Ty, V ∈ T) → (
        meet_values(Vx, Vy, V),
        reduce_meet(Tx, Ty, T)
    ),
    (Tx, V ∈ Ty, V ∈ T) →
        reduce_meet(Tx, Ty, T),
    (V ∈ Tx, Ty, V ∈ T) →
        reduce_meet(Tx, Ty, T),

    (meet([]), T, T),
    (T, meet([]), T),
    (meet([X|Xs]), meet(Y), Z) → (
        append(Xs, Y, Ts),
        reduce_meet(X, meet(Ts), Z)
    ),
    (meet([X]), Y, Z) →
        reduce_meet(X, Y, Z),
    (X, meet([Y]), Z) →
        reduce_meet(X, Y, Z),
    (meet([X|Xs]), Y, Z) → (
        append(Xs, [Y], Ts),
        reduce_meet(X, meet(Ts), Z)
    ),
    (X, meet([Y|Ys]), Z) → (
        reduce_meet(X, Y, T),
        reduce_meet(T, meet(Ys), Z)
    ),

    (X, join([Y]), Z) → 
        reduce_meet(X, Y, Z),
    (X, join([Y|Ys]), join([Z|Zs])) → (
        reduce_meet(X, Y, Z),
        reduce_meet(X, join(Ys), join(Zs))
    ),
    (join([X]), Y, Z) → 
        reduce_meet(X, Y, Z),
    (join([X|Xs]), Y, join([Z|Zs])) → (
        reduce_meet(X, Y, Z),
        reduce_meet(join(Xs), Y, join(Zs))
    ),

    (ref(X), Y, reduced(meet([ref(X), Y]))),
    (X, ref(Y), reduced(meet([X, ref(Y)]))),

    (prod(X), Y, Z) → fail,
    (X, prod(Y), Z) → fail
]) // M = bottom.

meet_values(V, V, V).

sizeof(P, S) :-
    ctypeof(P, T),
    sizeof_type(T, S).

size_sum(Sx, Sy, S) :- fail.

size_join(Sx, Sy, S) :- match((Sx, Sy, S), [
    (bottom, Y, Y),
    (X, bottom, X),
    (join(Xs), join(Ys), join(Zs)) →
        append(Xs, Ys, Zs),
    (X, join(Ys), join([X|Ys])),
    (join(Xs), Y, join([Y|Xs])),
    (X, Y, join([X, Y])) 
]).

size_mul(Sx, Sy, S) :- fail.
size_unify(Sx, Sy, S) :- fail.

size_constraint([], []).
size_constraint([P|Ps], [S|Ss]) :-
    (P, S) = (sizeof=C, hole=C) *->
        size_constraint(Ps, Ss)
    ;
        size_constraint(Ps, [S|Ss]).

sizeof_type(prod(Ts), S) :-
    maplist(
        [T, S] >> sizeof_type(T, S),
        Ts, Ss
    ),
    foldl(
        [Sx, Sy, S] >> size_sum(Sx, Sy, S),
        Ss, [], S
    ).

sizeof_type(join(Ts), S) :-
    maplist(
        [T, S] >> sizeof_type(T, S),
        Ts, Ss
    ),
    foldl(
        [Sx, Sy, S] >> size_join(Sx, Sy, S),
        Ss, bottom, S
    ).

sizeof_type(T, []) :-
    T = top ; T = integer.

sizeof_type(bottom, bottom).

sizeof_type(u(S, _), [hole=rint(S)]).
sizeof_type(_ ∈ T, S) :-
    sizeof_type(T, S).
sizeof_type(arr(T, N), S) :-
    sizeof_type(T, St),
    Sn = N,
    size_mul(St, Sn, S).
sizeof_type(T ⋅ _, S) :-
    sizeof_type(T, S).
sizeof_type(ref(Path), S) :-
    sizeof(Path, S).
sizeof_type(T ⋮ P, S) :-
    sizeof_type(T, Sx),
    size_constraint(P, Sy),
    size_unify(Sx, Sy, S).
