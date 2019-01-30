(define (square-list items)
  (define (iter things answer)
    (if (null? things)
        answer
	(iter (cdr things)
	      (append answer
		      (list (square (car things)))))))
  (iter items ()))

(define (square-list2 items)
  (define (iter things answer)
    (if (null? things)
        answer
	(iter (cdr things)
	      (cons (square (car things))
		    answer))))
  (iter items ()))

(define (square-list3 items)
  (define (iter things answer)
    (if (null? things)
        answer
	(iter (cdr things)
	      (cons answer
		    (square (car things))))))
  (iter items ()))
