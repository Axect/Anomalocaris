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

    ARMAT col(unsigned long k) {
        arma::mat m = this->to_mat();
        arma::mat c = m.col(k);
        return ARMAT(c);
    }

    ARMAT row(unsigned long k) {
        arma::mat m = this->to_mat();
        arma::mat r = m.row(k);
        return ARMAT(r);
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
}

