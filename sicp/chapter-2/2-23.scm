(define (for-each f ls)
  (define (iter ls1 val)
    (if (null? ls1)
        val
	(iter (cdr ls1) (f (car ls1)))))
  (iter ls ()))
