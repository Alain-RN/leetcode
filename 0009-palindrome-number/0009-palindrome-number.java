class Solution {
    public boolean isPalindrome(int x) {
        for(int i = 0; i < (""+x).length(); i++) {
            if( (""+x).charAt(i) != (""+x).charAt((""+x).length() - i - 1) ) return false;
        }
        return true;
    }
}