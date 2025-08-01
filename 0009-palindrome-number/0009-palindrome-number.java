class Solution {
    public boolean isPalindrome(int x) {
        String s = ""+x;
        int l = s.length();
        for(int i = 0; i < l; i++) {
            if( s.charAt(i) != s.charAt(l - i - 1) ) return false;
        }
        return true;
    }
}