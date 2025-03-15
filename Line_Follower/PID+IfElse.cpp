#define lir 2
#define lirm 3
#define cir 4
#define rirm 5
#define rir 6
#define m1 11
#define m2 12
#define m3 8
#define m4 7
#define en1 A1
#define en2 A2

//constants
float Kp = 29;
float Kd = 5;

//variables
float previous_error = 0;

void setup() {
  pinMode(lir, INPUT);
  pinMode(lirm, INPUT);
  pinMode(cir, INPUT);
  pinMode(rirm, INPUT);
  pinMode(rir, INPUT);
  pinMode(m1, OUTPUT);
  pinMode(m2, OUTPUT);
  pinMode(m3, OUTPUT);
  pinMode(m4, OUTPUT);
  pinMode(9, OUTPUT);
  pinMode(10, OUTPUT);
  Serial.begin(9600);
}

void loop() {
  int lir_read = digitalRead(lir);
  int lir_readm = digitalRead(lirm);
  int cir_read = digitalRead(cir);
  int rir_readm = digitalRead(rirm);
  int rir_read = digitalRead(rir);


  float error = calculateError(digitalRead(lir), digitalRead(lirm), digitalRead(cir), digitalRead(rirm), digitalRead(rir));
  float P = error;
  float D = error - previous_error;
  previous_error = error;

  float output = Kp * P + Kd * D;

  adjustMotors(output);

  if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == LOW))  // forward00000
  {
    forward();
  }
  else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == HIGH))  //right11111
  {
    forward();
  } else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == HIGH))  // forward11011
  {
    forward();
  }
  else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == HIGH))  //10001 was right
  {
    right();
  }
  else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == HIGH)) {
    // left00111
    left();
  } else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == HIGH))  // left01111
  {
    left();
  } else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == HIGH))  // left00011
  {
    left();
  } else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == HIGH))  // left00001
  {
    left();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == HIGH))  // left10111
  {
    left();
  }
  else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == LOW))  // right11110
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == LOW))  // right11100
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == LOW))  // right11000
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == LOW))  // right10000
  {
    right();
  } else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == LOW))  // 00010
  {
    left();
  }
  else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == LOW))  //00100
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == LOW))  //10100
  {
    left();
  }
  else if ((lir_read == LOW) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == LOW))  //01000
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == HIGH))  //10011 was right
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == HIGH))  //11001
  {
    right();
  }
  else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == HIGH))  //00101
  {
    left();
  } else if ((lir_read == LOW) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == LOW))  //00110
  {
    left();
  } else if ((lir_read == LOW) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == LOW) && (rir_read == HIGH))  //01001
  {
    right();
  } else if ((lir_read == LOW) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == HIGH))  //01011
  {
    left();
  } else if ((lir_read == LOW) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == LOW))  //01100
  {
    right();
  } else if ((lir_read == LOW) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == HIGH))  //01101
  {
    right();
  } else if ((lir_read == LOW) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == LOW))  //01110
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == LOW))  //10010
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == HIGH))  //10101
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == LOW) && (cir_read == HIGH) && (rir_readm == HIGH) && (rir_read == LOW))  //10110
  {
    right();
  } else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == LOW) && (rir_readm == HIGH) && (rir_read == LOW))  //11010
  {
    right();
  }
  else if ((lir_read == HIGH) && (lir_readm == HIGH) && (cir_read == HIGH) && (rir_readm == LOW) && (rir_read == HIGH))  // right11101 was right
  {
    right();
  }
}

float calculateError(int ll, int ml, int m, int mr, int rr) {
   
    // Adjust based on your sensor arrangement
    int position = ll * 0 + ml * 10 + m * 20 + mr * 30 + rr * 40;
    int total = ll + ml + m + mr + rr;
    if (total == 0) {
        return 0; // Line not detected
    }
    return position / total - 20; // Center position is 2 (middle sensor)
}

void adjustMotors(float output){
  int basespeed;
  float turnIntensity = abs(output);
  if (turnIntensity < 0.1){
    basespeed = 220;
  } else {
    basespeed = 0;
  }

  int rightSpeed = basespeed - output;
  int leftSpeed = basespeed + output;

//  analogWrite(en1, rightSpeed);
//  analogWrite(en2, leftSpeed);

// Set motor speeds
    analogWrite(en1, constrain(abs(leftSpeed), 140, 230));
    analogWrite(en2, constrain(abs(rightSpeed), 140, 230));


      // Set motor directions
    digitalWrite(m3, rightSpeed > 0 ? HIGH : LOW);
    digitalWrite(m4, rightSpeed > 0 ? LOW : HIGH);
    digitalWrite(m1, leftSpeed > 0 ? HIGH : LOW);
    digitalWrite(m2, leftSpeed > 0 ? LOW : HIGH);

    Serial.print(output);
    Serial.print(", ");
    Serial.print(basespeed);
    Serial.print(", ");
    Serial.print(rightSpeed);
    Serial.print(", ");
    Serial.println(leftSpeed);

}
void forward() {
  digitalWrite(m1, HIGH);
  digitalWrite(m2, LOW);
  digitalWrite(m3, LOW);
  digitalWrite(m4, HIGH);
}
void backward() {
  digitalWrite(m1, LOW);
  digitalWrite(m2, HIGH);
  digitalWrite(m3, HIGH);
  digitalWrite(m4, LOW);
}
void right() {
  digitalWrite(m1, HIGH);
  digitalWrite(m2, LOW);
  digitalWrite(m3, HIGH);
  digitalWrite(m4, LOW);
}
void left() {
  digitalWrite(m1, LOW);
  digitalWrite(m2, HIGH);
  digitalWrite(m3, LOW);
  digitalWrite(m4, HIGH);
}
