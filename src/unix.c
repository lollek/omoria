#include <errno.h>
#include <string.h>
#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>
#include <pwd.h>

/* Find a default user name from the system. */
void user_name(char *buf)
{
  const char *p = getlogin();
  if (p != NULL && p[0])
    (void)strcpy(buf, p);
  else {
    const struct passwd *pwline = getpwuid(getuid());
    if (pwline != NULL)
      (void)strcpy(buf, pwline->pw_name);
  }
  if (!buf[0])
    (void)strcpy(buf, "X"); /* Gotta have some name */
}

/* expands a tilde at the beginning of a file name to a users home
   directory */
int tilde(const char *file, char *exp)
{
  *exp = '\0';
  if (file != NULL) {
    if (*file == '~') {
      char user[128];
      const struct passwd *pw = NULL;
      unsigned int i = 0;

      user[0] = '\0';
      file++;
      while (*file != '/' && i < sizeof(user))
        user[i++] = *file++;
      user[i] = '\0';
      if (i == 0) {
        const char *login = getlogin();
        if (login != NULL)
          (void)strcpy(user, login);
        else if ((pw = getpwuid(getuid())) == NULL)
          return 0;
      }
      if (pw == NULL && (pw = getpwnam(user)) == NULL)
        return 0;
      (void)strcpy(exp, pw->pw_dir);
    }
    (void)strcat(exp, file);
    return 1;
  }
  return 0;
}
