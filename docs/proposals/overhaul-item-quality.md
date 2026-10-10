## Overhaul item quality

Related info:
- src/generate_item/item_template.rs

Initially we had cursed, normal, magical and unique items. Now cursed if basically removed from the code,
while low and high quality are added. The code still has some cursed item logic, which probably is no longer possible to hit.
And there is no difference in item when its low or high quality.

Find some inspiration on a good way to use item quality. A good starting point is Diablo II.
Then hash out an implementation of it. Better to start slow and evolve it gradually