LCM for 23, 19, 13, 17 = 96577

Najmniejszy wspolny dzielnik danego zakresu liczb mieści się w zakresie od
najwiekszego elementu do iloczynu wszystkich elementow.

Czyli w przypadku powyżej od: 23 do 96577. Sprawdzanie wszystkiego wyzej to powtórki i niepotrzebne operacje.

Czyli:

Monkey 0:
Starting items: 79, 98
Operation: new = old \* 19
--> new = new % 96577 !!!!!
Test: divisible by 23
If true: throw to monkey 2
If false: throw to monkey 3
