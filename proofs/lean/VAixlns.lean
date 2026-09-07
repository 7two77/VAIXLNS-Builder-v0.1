import VAixlns.Basic

namespace VAixlns

-- Экспортируем все теоремы
export Basic (GenesisImmutable, Replayable, State, Kernel, Runtime, 
              RuntimeRequiresKernel, kernel_implies_runtime_correct,
              dependency_graph_acyclic, deterministic_execution,
              evidence_hash_valid, finality_gate_correct)

end VAixlns
