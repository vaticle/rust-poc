
using namespace std;

namespace operations_research {
    using MPSolverResultStatus = MPSolver::ResultStatus;

    std::unique_ptr<MPSolver> new_mpsolver() {
      return std::unique_ptr<MPSolver>(MPSolver::CreateSolver("SCIP"));
    }
}