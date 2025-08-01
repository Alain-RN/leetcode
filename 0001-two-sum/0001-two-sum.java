class Solution {
    public int[] twoSum(int[] nums, int target) {
        int j = 0;
        for(int i = 0; i < nums.length; i++) {
            while( j < nums.length && j != i ) {
                if( ( nums[j] + nums[i] ) == target ) {
                    return new int[]{i, j};
                }
                j++;
            }
            j = 0;
        }
        return new int[]{0, 0};
    }
}