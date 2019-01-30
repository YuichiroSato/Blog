(define (reverse ls)
  (define (iter ls1 result)
    (if (null? ls1)
        result
	(iter (cdr ls1) (cons (car ls1) result))))
  (iter ls '()))
