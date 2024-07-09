int main() {
    int x = 3; // a normal comment
    // line seperate comment
    int y /* hehe */ = /* more wack */ 3 /* :D */;
    // another line comment
    /* how about a longer
     multi
line
                           comme
nt
                 */
     int z = 4 // this should still be fine
            + 10 /* and this
          too?
           */
            - 4 /* wait what if i do math here
            + 10000
            */
             ;
     int w = /* wait this might be funny */
              4 * 3 /* == 4 */;
}
