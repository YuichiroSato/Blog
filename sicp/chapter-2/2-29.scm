;;
;;
;;  2       4      4            8 weight          3
;;  |       |      |            |                 |
;;  |       |      |            |                 |
;;  |       --------            |                 |
;;  |           |               |                 |
;;  -------------               |                 |
;;           |                  |                 |
;;           |                  |                 |
;;           --------------------                 |
;;                   |                            |
;;                   | right branch               | left branch
;;                   |                            |
;;                   ------------------------------
;;                        | <--------------------->
;;                        |          length
;;
;;  -------------------------------------------------> x
;;                        0
;;
;;  moment = length * weight
;;
;;

(define (make-mobile left right)
  (list left right))

(define (make-branch length structure)
  (list length structure))

(define (left-branch mobile)
  (car mobile))

(define (right-branch mobile)
  (car (cdr mobile)))

(define (branch-length branch)
  (car branch))

(define (branch-structure branch)
  (car (cdr branch)))

(define (is-weight? structure)
  (not (pair? structure)))

(define (left-structure mobile)
  (branch-structure (left-branch mobile)))

(define (right-structure mobile)
  (branch-structure (right-branch mobile)))

(define (total-weight mobile)
  (define (check s)
    (if (is-weight? s) s (total-weight s)))
  (let ((l (left-structure mobile))
	(r (right-structure mobile)))
    (+ (check l) (check r))))

(define (is-balanced? mobile)
  (let ((ls (left-structure mobile))
	(ll (branch-length (left-branch mobile)))
	(rs (right-structure mobile))
        (rl (branch-length (right-branch mobile))))
    (let ((lw (is-weight? ls))
	  (rw (is-weight? rs)))
      (cond ((and lw rw) (= (* ll ls) (* rl rs)))
	    (rw (and (is-balanced? ls) (= (* ll (total-weight ls)) (* rl rs))))
	    (lw (and (is-balanced? rs) (= (* ll ls) (* rl (total-weight rs)))))
	    (else (and (is-balanced? rs) (is-balanced? ls) (= (* ll (total-weight ls)) (* rl (total-weight rs)))))))))

(define mobile1 (make-mobile (make-branch 2 3) (make-branch 2 3)))

(define mobile2 (make-mobile (make-branch 10 mobile1) (make-branch 10 mobile1)))

(define mobile3 (make-mobile (make-branch 15 mobile1) (make-branch 10 mobile1)))

(define mobile4 (make-mobile (make-branch 2 3)
			     (make-branch 1 (make-mobile (make-branch 1 3)
							 (make-branch 1 3)))))
