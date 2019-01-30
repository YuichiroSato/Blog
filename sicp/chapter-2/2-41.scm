(define (unique-pairs n)
  (define (iter is result)
    (if (null? is)
        result
	(let ((a (car is)))
	     (iter (cdr is) (append result (map (lambda (i) (cons a i)) is))))))
  (if (< n 0)
      ()
      (iter (one-to-n n) ())))

(define (triples n)
  (let ((is (one-to-n n)))
    (fold-left append ()
	       (fold-left append () 
			  (map (lambda (i) (map (lambda (j) (map (lambda (k) (list i j k))
								 is))
						is))
			       is)))))

(define (orderd-triples n)
  (filter is-orderd (triples n)))

(define (one-to-n n)
  (define (iter i result)
    (if (> i n)
        (reverse result)
	(iter (+ i 1) (cons i result))))
  (iter 1 n))

(define (is-orderd ls)
  (cond ((null? ls) #t)
	((< (length ls) 2) #t)
	(else (and (< (car ls) (cadr ls)) (is-orderd (cdr ls))))))

(define (sum ls)
  (fold-left + 0 ls))

(define (find-total s n)
  (filter (lambda (t) (= s (sum t))) (orderd-triples n)))
