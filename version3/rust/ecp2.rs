/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use xxx::rom;
use xxx::big;
use xxx::fp2::FP2;
use xxx::big::BIG;

pub struct ECP2 {
	x:FP2,
	y:FP2,
	z:FP2,
	inf: bool
}

#[allow(non_snake_case)]
impl ECP2 {

	pub fn new() -> ECP2 {
		ECP2 {
				x: FP2::new(),
				y: FP2::new_int(1),
				z: FP2::new(),
				inf: true
		}
	}
#[allow(non_snake_case)]
/* construct this from (x,y) - but set to O if not on curve */
	pub fn new_fp2s(ix:&FP2,iy:&FP2) -> ECP2 {
		let mut E=ECP2::new();
		E.x.copy(&ix);
		E.y.copy(&iy);
		E.z.one();

		let mut rhs=ECP2::rhs(&mut E.x);
		let mut y2=FP2::new_copy(&E.y);
		y2.sqr();
		if y2.equals(&mut rhs) {
			E.inf=false;
		} else {E.x.zero();E.inf=true}
		return E;
}

/* construct this from x - but set to O if not on curve */
	pub fn new_fp2(ix:&FP2) -> ECP2 {	
		let mut E=ECP2::new();
		E.x.copy(&ix);
		E.y.one();
		E.z.one();

		let mut rhs=ECP2::rhs(&mut E.x);
		if rhs.sqrt() {
			E.y.copy(&rhs);
			E.inf=false;
		} else {E.x.zero();E.inf=true}
		return E;
	}

/* Test this=O? */
	pub fn is_infinity(&self) -> bool {
		if self.inf {return true}
		let mut xx=FP2::new_copy(&self.x);
		let mut zz=FP2::new_copy(&self.z);
		return xx.iszilch() && zz.iszilch();
	}

/* copy self=P */
	pub fn copy(&mut self,P: &ECP2) {
		self.x.copy(&P.x);
		self.y.copy(&P.y);
		self.z.copy(&P.z);
		self.inf=P.inf;
	}

/* set self=O */
	pub fn inf(&mut self) {
		self.inf=true;
		self.x.zero();
		self.y.one();
		self.z.zero();
	}

/* set self=-self */
	pub fn neg(&mut self) {
	//	if self.is_infinity() {return}
		self.y.norm(); self.y.neg(); self.y.norm();
	}	

/* Conditional move of Q to self dependant on d */
	pub fn cmove(&mut self,Q: &ECP2,d: isize) {
		self.x.cmove(&Q.x,d);
		self.y.cmove(&Q.y,d);
		self.z.cmove(&Q.z,d);

		let bd:bool;
		if d==0 {bd=false}
		else {bd=true}

		self.inf=(self.inf!=(self.inf!=Q.inf)&&bd);
	}

/* return 1 if b==c, no branching */
	fn teq(b: i32,c: i32) -> isize {
		let mut x=b^c;
		x-=1;  // if x=0, x now -1
		return ((x>>31)&1) as isize;
	}

/* Constant time select from pre-computed table */
	pub fn selector(&mut self,W: &[ECP2],b: i32) {
		let mut MP=ECP2::new(); 
		let m=b>>31;
		let mut babs=(b^m)-m;

		babs=(babs-1)/2;

		self.cmove(&W[0],ECP2::teq(babs,0));  // conditional move
		self.cmove(&W[1],ECP2::teq(babs,1));
		self.cmove(&W[2],ECP2::teq(babs,2));
		self.cmove(&W[3],ECP2::teq(babs,3));
		self.cmove(&W[4],ECP2::teq(babs,4));
		self.cmove(&W[5],ECP2::teq(babs,5));
		self.cmove(&W[6],ECP2::teq(babs,6));
		self.cmove(&W[7],ECP2::teq(babs,7));
 
		MP.copy(self);
		MP.neg();
		self.cmove(&MP,(m&1) as isize);
	}	

/* Test if P == Q */
	pub fn equals(&mut self,Q :&mut ECP2) -> bool {
		if self.is_infinity() && Q.is_infinity() {return true}
		if self.is_infinity() || Q.is_infinity() {return false}

		let mut a=FP2::new_copy(&self.x);
		let mut b=FP2::new_copy(&Q.x); 

		a.mul(&Q.z);
		b.mul(&self.z);
		if !a.equals(&mut b) {return false}
		a.copy(&self.y); a.mul(&Q.z);
		b.copy(&Q.y); b.mul(&self.z);
		if !a.equals(&mut b) {return false}

		return true;
	}

/* set to Affine - (x,y,z) to (x,y) */
	pub fn affine(&mut self) {
		if self.is_infinity() {return}
		let mut one=FP2::new_int(1);
		if self.z.equals(&mut one) {return}
		self.z.inverse();

		self.x.mul(&self.z); self.x.reduce(); 
		self.y.mul(&self.z); self.y.reduce();
		self.z.copy(&one);
	}

/* extract affine x as FP2 */
	pub fn getx(&mut self) -> FP2 {
		self.affine();
		return FP2::new_copy(&self.x);
	}

/* extract affine y as FP2 */
	pub fn gety(&mut self) -> FP2 {
		self.affine();
		return FP2::new_copy(&self.y);
	}

/* extract projective x */
	pub fn getpx(&self) -> FP2 {
		return FP2::new_copy(&self.x);
	}
/* extract projective y */
	pub fn getpy(&self) -> FP2 {
		return FP2::new_copy(&self.y);
	}
/* extract projective z */
	pub fn getpz(&self) -> FP2 {
		return FP2::new_copy(&self.z);
	}

/* convert to byte array */
	pub fn tobytes(&mut self,b: &mut [u8]) {
		let mut t:[u8;big::MODBYTES as usize]=[0;big::MODBYTES as usize];
		let mb=big::MODBYTES as usize;

		self.affine();
		self.x.geta().tobytes(&mut t);
		for i in 0..mb { b[i]=t[i]}
		self.x.getb().tobytes(&mut t);
		for i in 0..mb { b[i+mb]=t[i]}

		self.y.geta().tobytes(&mut t);
		for i in 0..mb {b[i+2*mb]=t[i]}
		self.y.getb().tobytes(&mut t);
		for i in 0..mb {b[i+3*mb]=t[i]}
	}

/* convert from byte array to point */
	pub fn frombytes(b: &[u8]) -> ECP2 {
		let mut t:[u8;big::MODBYTES as usize]=[0;big::MODBYTES as usize];
		let mb=big::MODBYTES as usize;

		for i in 0..mb {t[i]=b[i]}
		let mut ra=BIG::frombytes(&t);
		for i in 0..mb {t[i]=b[i+mb]}
		let mut rb=BIG::frombytes(&t);
		let rx=FP2::new_bigs(&ra,&rb);

		for i in 0..mb {t[i]=b[i+2*mb]}
		ra.copy(&BIG::frombytes(&t));
		for i in 0..mb {t[i]=b[i+3*mb]}
		rb.copy(&BIG::frombytes(&t));
		let ry=FP2::new_bigs(&ra,&rb);

		return ECP2::new_fp2s(&rx,&ry);
	}

/* convert this to hex string */
	pub fn tostring(&mut  self) -> String {
		if self.is_infinity() {return String::from("infinity")}
		self.affine();
		return format!("({},{})",self.x.tostring(),self.y.tostring());
}	

/* Calculate RHS of twisted curve equation x^3+B/i */
	pub fn rhs(x:&mut FP2) -> FP2 {
		x.norm();
		let mut r=FP2::new_copy(x);
		r.sqr();
		let mut b=FP2::new_big(&BIG::new_ints(&rom::CURVE_B));
		b.div_ip();
		r.mul(x);
		r.add(&b);

		r.reduce();
		return r;
	}

/* self+=self */
	pub fn dbl(&mut self) -> isize {
		if self.inf {return -1}

		let mut iy=FP2::new_copy(&self.y);
		iy.mul_ip(); iy.norm();

		let mut t0=FP2::new_copy(&self.y);                  //***** Change 
		t0.sqr();  t0.mul_ip();   
		let mut t1=FP2::new_copy(&iy);  
		t1.mul(&self.z);
		let mut t2=FP2::new_copy(&self.z);
		t2.sqr();

		self.z.copy(&t0);
		self.z.add(&t0); self.z.norm(); 
		self.z.dbl(); 
		self.z.dbl(); 
		self.z.norm();  

		t2.imul(3*rom::CURVE_B_I); 

		let mut x3=FP2::new_copy(&t2);
		x3.mul(&self.z); 

		let mut y3=FP2::new_copy(&t0);   

		y3.add(&t2); y3.norm();
		self.z.mul(&t1);
		t1.copy(&t2); t1.add(&t2); t2.add(&t1); t2.norm();  
		t0.sub(&t2); t0.norm();                           //y^2-9bz^2
		y3.mul(&t0); y3.add(&x3);                          //(y^2+3z*2)(y^2-9z^2)+3b.z^2.8y^2
		t1.copy(&self.x); t1.mul(&iy);						//
		self.x.copy(&t0); self.x.norm(); self.x.mul(&t1); self.x.dbl();       //(y^2-9bz^2)xy2

		self.x.norm(); 
		self.y.copy(&y3); self.y.norm();

		return 1;

	}

/* self+=Q - return 0 for add, 1 for double, -1 for O */
	pub fn add(&mut self,Q:&ECP2) -> isize {
		if self.inf {
			self.copy(Q);
			return -1;
		}
		if Q.inf {return -1}


		let b=3*rom::CURVE_B_I;
		let mut t0=FP2::new_copy(&self.x);
		t0.mul(&Q.x);         // x.Q.x
		let mut t1=FP2::new_copy(&self.y);
		t1.mul(&Q.y);		 // y.Q.y

		let mut t2=FP2::new_copy(&self.z);
		t2.mul(&Q.z);
		let mut t3=FP2::new_copy(&self.x);
		t3.add(&self.y); t3.norm();          //t3=X1+Y1
		let mut t4=FP2::new_copy(&Q.x);            
		t4.add(&Q.y); t4.norm();			//t4=X2+Y2
		t3.mul(&t4);						//t3=(X1+Y1)(X2+Y2)
		t4.copy(&t0); t4.add(&t1);		//t4=X1.X2+Y1.Y2

		t3.sub(&t4); t3.norm(); t3.mul_ip();  t3.norm();         //t3=(X1+Y1)(X2+Y2)-(X1.X2+Y1.Y2) = X1.Y2+X2.Y1

		t4.copy(&self.y);                    
		t4.add(&self.z); t4.norm();			//t4=Y1+Z1
		let mut x3=FP2::new_copy(&Q.y);
		x3.add(&Q.z); x3.norm();			//x3=Y2+Z2

		t4.mul(&x3);						//t4=(Y1+Z1)(Y2+Z2)
		x3.copy(&t1);					//
		x3.add(&t2);						//X3=Y1.Y2+Z1.Z2
	
		t4.sub(&x3); t4.norm(); t4.mul_ip(); t4.norm();          //t4=(Y1+Z1)(Y2+Z2) - (Y1.Y2+Z1.Z2) = Y1.Z2+Y2.Z1

		x3.copy(&self.x); x3.add(&self.z); x3.norm();	// x3=X1+Z1
		let mut y3=FP2::new_copy(&Q.x);				
		y3.add(&Q.z); y3.norm();				// y3=X2+Z2
		x3.mul(&y3);							// x3=(X1+Z1)(X2+Z2)
		y3.copy(&t0);
		y3.add(&t2);							// y3=X1.X2+Z1+Z2
		y3.rsub(&x3); y3.norm();				// y3=(X1+Z1)(X2+Z2) - (X1.X2+Z1.Z2) = X1.Z2+X2.Z1

		t0.mul_ip(); t0.norm(); // x.Q.x
		t1.mul_ip(); t1.norm(); // y.Q.y

		x3.copy(&t0); x3.add(&t0); 
		t0.add(&x3); t0.norm();
		t2.imul(b); 	

		let mut z3=FP2::new_copy(&t1); z3.add(&t2); z3.norm();
		t1.sub(&t2); t1.norm(); 
		y3.imul(b); 

		x3.copy(&y3); x3.mul(&t4); t2.copy(&t3); t2.mul(&t1); x3.rsub(&t2);
		y3.mul(&t0); t1.mul(&z3); y3.add(&t1);
		t0.mul(&t3); z3.mul(&t4); z3.add(&t0);

		self.x.copy(&x3); self.x.norm(); 
		self.y.copy(&y3); self.y.norm();
		self.z.copy(&z3); self.z.norm();

		return 0;
	}

/* set this-=Q */
	pub fn sub(&mut self,Q :&ECP2) -> isize {
		let mut NQ=ECP2::new(); NQ.copy(Q);
		NQ.neg();
		let d=self.add(&NQ);
		return d;
	}

/* set this*=q, where q is Modulus, using Frobenius */
	pub fn frob(&mut self,x:&FP2) {
	 	if self.inf {return}
		let mut x2=FP2::new_copy(x);
		x2.sqr();
		self.x.conj();
		self.y.conj();
		self.z.conj();
		self.z.reduce();
		self.x.mul(&x2);
		self.y.mul(&x2);
		self.y.mul(x);
	}

/* self*=e */
	pub fn mul(&self,e: &BIG) -> ECP2 {
/* fixed size windows */
		let mut mt=BIG::new();
		let mut t=BIG::new();
		let mut P=ECP2::new();
		let mut Q=ECP2::new();
		let mut C=ECP2::new();

		if self.is_infinity() {return P}

		let mut W:[ECP2;8]=[ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new()];

		const CT:usize=1+(big::NLEN*(big::BASEBITS as usize)+3)/4;
		let mut w:[i8;CT]=[0;CT]; 

	//	self.affine();

/* precompute table */
		Q.copy(&self);
		Q.dbl();
		
		W[0].copy(&self);

		for i in 1..8 {
			C.copy(&W[i-1]);
			W[i].copy(&C);
			W[i].add(&mut Q);
		}

/* make exponent odd - add 2P if even, P if odd */
		t.copy(&e);
		let s=t.parity();
		t.inc(1); t.norm(); let ns=t.parity(); mt.copy(&t); mt.inc(1); mt.norm();
		t.cmove(&mt,s);
		Q.cmove(&self,ns);
		C.copy(&Q);

		let nb=1+(t.nbits()+3)/4;

/* convert exponent to signed 4-bit window */
		for i in 0..nb {
			w[i]=(t.lastbits(5)-16) as i8;
			t.dec(w[i] as isize); t.norm();
			t.fshr(4);	
		}
		w[nb]=(t.lastbits(5)) as i8;
		
		P.copy(&W[((w[nb] as usize) -1)/2]);
		for i in (0..nb).rev() {
			Q.selector(&W,w[i] as i32);
			P.dbl();
			P.dbl();
			P.dbl();
			P.dbl();
			P.add(&mut Q);
		}
		P.sub(&mut C);
		P.affine();
		return P;
	}

/* P=u0.Q0+u1*Q1+u2*Q2+u3*Q3 */
	pub fn mul4(Q: &mut [ECP2],u: &[BIG]) -> ECP2 {
		let mut a:[i8;4]=[0;4];
		let mut T=ECP2::new();
		let mut C=ECP2::new();
		let mut P=ECP2::new();

		let mut W:[ECP2;8]=[ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new(),ECP2::new()];

		let mut mt=BIG::new();

		let mut t:[BIG;4]=[BIG::new_copy(&u[0]),BIG::new_copy(&u[1]),BIG::new_copy(&u[2]),BIG::new_copy(&u[3])];

		const CT:usize=1+big::NLEN*(big::BASEBITS as usize);
		let mut w:[i8;CT]=[0;CT];

		for i in 0..4 {
			Q[i].affine();
		}

/* precompute table */

		W[0].copy(&Q[0]); W[0].sub(&mut Q[1]);
		C.copy(&W[0]); W[1].copy(&C);
		W[2].copy(&C);
		W[3].copy(&C);
		W[4].copy(&Q[0]); W[4].add(&mut Q[1]);
		C.copy(&W[4]); W[5].copy(&C);
		W[6].copy(&C);
		W[7].copy(&C);

		T.copy(&Q[2]); T.sub(&mut Q[3]);
		W[1].sub(&mut T);
		W[2].add(&mut T);
		W[5].sub(&mut T);
		W[6].add(&mut T);
		T.copy(&Q[2]); T.add(&mut Q[3]);
		W[0].sub(&mut T);
		W[3].add(&mut T);
		W[4].sub(&mut T);
		W[7].add(&mut T);

/* if multiplier is even add 1 to multiplier, and add P to correction */
		mt.zero(); C.inf();
		for i in 0..4 {
			if t[i].parity()==0 {
				t[i].inc(1); t[i].norm();
				C.add(&mut Q[i]);
			}
			mt.add(&t[i]); mt.norm();
		}

		let nb=1+mt.nbits();

/* convert exponent to signed 1-bit window */
		for j in 0..nb {
			for i in 0..4 {
				a[i]=(t[i].lastbits(2)-2) as i8;
				t[i].dec(a[i] as isize); t[i].norm();
				t[i].fshr(1);
			}
			w[j]=8*a[0]+4*a[1]+2*a[2]+a[3];
		}
		w[nb]=(8*t[0].lastbits(2)+4*t[1].lastbits(2)+2*t[2].lastbits(2)+t[3].lastbits(2)) as i8;

		P.copy(&W[((w[nb] as usize)-1)/2]);  
		for i in (0..nb).rev() {
			T.selector(&W,w[i] as i32);
			P.dbl();
			P.add(&mut T);
		}
		P.sub(&mut C); /* apply correction */

		P.affine();
		return P;
	}

}
/*
fn main()
{
	let mut r=BIG::new_ints(&rom::MODULUS);

	let pxa=BIG::new_ints(&rom::CURVE_PXA);
	let pxb=BIG::new_ints(&rom::CURVE_PXB);
	let pya=BIG::new_ints(&rom::CURVE_PYA);
	let pyb=BIG::new_ints(&rom::CURVE_PYB);

	let fra=BIG::new_ints(&rom::CURVE_FRA);
	let frb=BIG::new_ints(&rom::CURVE_FRB);

	let mut f=FP2::new_bigs(&fra,&frb);

	let px=FP2::new_bigs(&pxa,&pxb);
	let py=FP2::new_bigs(&pya,&pyb);

	let mut P=ECP2::new_fp2s(&px,&py);

	println!("P= {}",P.tostring());

	P=P.mul(&mut r);
	println!("P= {}",P.tostring());

	let mut  Q=ECP2::new_fp2s(&px,&py);
	Q.frob(&mut f);
	println!("Q= {}",Q.tostring());
}
*/