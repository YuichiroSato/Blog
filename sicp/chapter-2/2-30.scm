(define (square-tree tree)
  (cond ((null? tree) ())
	((pair? tree)
           (cons (square-tree (car tree)) (square-tree (cdr tree))))
        (else (square tree))))

(define (square-tree2 tree)
  (if (pair? tree)
      (map square-tree2 tree)
      (square tree)))
