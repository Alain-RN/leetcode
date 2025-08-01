/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode l_t1 = l1;
        ListNode l_t2 = l2;
        int r = 0;
        ListNode l_sol = new ListNode();
        ListNode sol = new ListNode();
        boolean first = true;


        while (l_t1 != null || l_t2 != null) {

            int x = (l_t1 != null) ? l_t1.val : 0;
            int y = (l_t2 != null) ? l_t2.val : 0;
            
            if( first ) {
                l_sol.val = (x + y + r) % 10;
                first = false;
            } else {
                l_sol = new ListNode( (x + y + r) % 10, l_sol);
            }

            if ((x + y + r) >= 10) {
                r = 1;
            } else {
                r = 0;
            }

            if (l_t1 != null) {
                l_t1 = l_t1.next;
            }

            if (l_t2 != null) {
                l_t2 = l_t2.next;
            }
        }
        if (r == 1) {
            l_sol = new ListNode(r, l_sol);
        }
        first = true;

        while(l_sol != null) {
            if( first ) {
                sol.val = l_sol.val;
                first = false;
            } else {
                sol = new ListNode( l_sol.val , sol);
            }

            l_sol = l_sol.next;
        }


        return sol;
    }
}