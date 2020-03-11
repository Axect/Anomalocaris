#include <armadillo>

// Wrapper struct for Armadillo mat
extern "C" struct ARMAT {
    double *data;
    unsigned long nrow;
    unsigned long ncol;

    ARMAT(arma::mat &m) {
        data = m.memptr();
        nrow = m.n_rows;
        ncol = m.n_cols;
    }

    arma::mat to_mat() {
        return arma::mat(data, nrow, ncol);
    }

};

extern "C" {
    // Basic
    ARMAT zeros_(unsigned long i, unsigned long j) {
        arma::mat z = arma::zeros(i, j);
        return ARMAT(z);
    }

    double det_(ARMAT* m) {
        return arma::det(m->to_mat());
    }

    ARMAT matmul_(ARMAT* m, ARMAT* n) {
        arma::mat a = m->to_mat();
        arma::mat b = n->to_mat();
        double* data = static_cast<double*>(malloc(sizeof(double) * m->nrow * n->ncol));
        arma::mat c(data, m->nrow, n->ncol);
        c = a * b;
        return ARMAT(c);
    }

    ARMAT col_(ARMAT* arm, unsigned long k) {
        arma::mat m = arm->to_mat();
        double* data = static_cast<double*>(malloc(sizeof(double) * arm->nrow));
        arma::mat c(data, arm->nrow, 1);
        c = m.col(k);
        return ARMAT(c);
    }

    ARMAT row_(ARMAT* arm, unsigned long k) {
        arma::mat m = arm->to_mat();
        arma::mat r = m.row(k);
        return ARMAT(r);
    }
}

