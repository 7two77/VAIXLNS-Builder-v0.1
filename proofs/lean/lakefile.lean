import Lake
open Lake DSL

package «vaixlns-proofs» where
  -- add package configuration options here

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git"

@[default_target]
lean_lib «VAixlns» where
  -- add library configuration options here
