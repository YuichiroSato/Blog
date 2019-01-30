(define (accumulate op initial sequence)
  (if (null? sequence)
      initial
      (op (car sequence)
	  (accumulate op initial (cdr sequence)))))

(define (accumulate-n op init seqs)
  (cond ((null? seqs) (list ()))
	((null? (cdr seqs)) (list (accumulate op init (car seqs))))
	(else (cons (accumulate op init (car seqs))
		    (accumulate-n op init (cdr seqs))))))
