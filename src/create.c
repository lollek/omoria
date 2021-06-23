/* create.c */
/* Ever want to create the perfect character? Here is where you can cheat. */

#include "imoria.h"

void set_gem_values()
{
	long count;

	ENTER(("set_gem_values", ""));

	for (count = 1; count <= MAX_OBJECTS; count++) {
		/*with object_list[count] do*/
		if ((strstr(object_list[count].name, "Finely cut") != NULL) &&
		    (strstr(object_list[count].name, "of") != NULL)) {

			if (strstr(object_list[count].name, "Amber") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Agate") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Alexandrite") !=
			    NULL) {
				object_list[count].cost += 5000;
			}
			/* amethyst was misspelled as "amathyst".  2/15/00 JEB
			 */
			if (strstr(object_list[count].name, "Amethyst") !=
			    NULL) {
				object_list[count].cost += 2000;
			}
			if (strstr(object_list[count].name, "Antlerite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Aquamarine") !=
			    NULL) {
				object_list[count].cost += 6000;
			}
			if (strstr(object_list[count].name, "Argentite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Azurite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Beryl") != NULL) {
				object_list[count].cost += 2000;
			}
			if (strstr(object_list[count].name, "Bloodstone") !=
			    NULL) {
				object_list[count].cost += 3500;
			}
			if (strstr(object_list[count].name, "Calcite") !=
			    NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Carnelian") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Coral") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Corundum") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Cryolite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Diamond") !=
			    NULL) {
				object_list[count].cost += 35000;
			}
			if (strstr(object_list[count].name, "Diorite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Emerald") !=
			    NULL) {
				object_list[count].cost += 20000;
			}
			if (strstr(object_list[count].name, "Flint") != NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Fluorite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Gabbro") != NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Garnet") != NULL) {
				object_list[count].cost += 6500;
			}
			if (strstr(object_list[count].name, "Granite") !=
			    NULL) {
				object_list[count].cost += 500;
			}
			if (strstr(object_list[count].name, "Gypsum") != NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Hematite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Jade") != NULL) {
				object_list[count].cost += 12000;
			}
			if (strstr(object_list[count].name, "Jasper") != NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Kryptonite") !=
			    NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Lapus lazuli") !=
			    NULL) {
				object_list[count].cost += 4500;
			}
			if (strstr(object_list[count].name, "Limestone") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Malachite") !=
			    NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Manganite") !=
			    NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Marble") != NULL) {
				object_list[count].cost += 5500;
			}
			if (strstr(object_list[count].name, "Mica") != NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Moonstone") !=
			    NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Neptunite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Obsidian") !=
			    NULL) {
				object_list[count].cost += 2500;
			}
			if (strstr(object_list[count].name, "Onyx") != NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Opal") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Pearl") != NULL) {
				object_list[count].cost += 11500;
			}
			if (strstr(object_list[count].name, "Pyrite") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Quartz") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Quartzite") !=
			    NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Rhodonite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Rhyolite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Ruby") != NULL) {
				object_list[count].cost += 14500;
			}
			if (strstr(object_list[count].name, "Sapphire") !=
			    NULL) {
				object_list[count].cost += 14500;
			}
			if (strstr(object_list[count].name, "Sphalerite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Staurolite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Tiger eye") !=
			    NULL) {
				object_list[count].cost += 8500;
			}
			if (strstr(object_list[count].name, "Topaz") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Turquoise") !=
			    NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Zircon") != NULL) {
				object_list[count].cost += 1000;
			}
		} /* end if (finely cut) */
	}	  /* end for */

	LEAVE("set_gem_values", "");
}
