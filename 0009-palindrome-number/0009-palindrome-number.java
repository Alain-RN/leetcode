class Solution {
    public boolean isPalindrome(int x) {
        String s = String.valueOf(x);
        int i = 0;
        while( i < s.length() && s.charAt(i) == s.charAt(s.length() - i - 1) ) {
            i++;
        }
        return i == s.length();
    }
}