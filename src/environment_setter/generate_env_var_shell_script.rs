use std::{
    fs::{self, File},
    path::PathBuf,
};

use colored::Colorize;

use super::env_setter::EnvKeyValue;
use std::io::Write;

pub fn generate_script(env_key_value: EnvKeyValue) {
    let env_name: String = env_key_value.env_name;
    let env_key: String = env_key_value.env_key;
    let env_value: String = env_key_value.env_value;

    let shell_script: String = format!(
        r#"
      
      # Colors
      # Define no color escape code (reset to default)
      RESET='\033[0m'

      # Define colors escape code
    
      BLACK_BACKGROUND='\033[0;40m'
      WHITE_BACKGROUND='\033[0;47m'

      WHITE_FOREGROUND='\033[0;37m'
      RED_FOREGROUND='\033[0;31m'
      
      # Bold Foreground
      BOLD_GREEN_FOREGROUND='\033[1;32m'
      BOLD_WHITE_FOREGROUND='\033[1;37m'
      BOLD_RED_FOREGROUND='\033[1;31m'


      ENV_NAME="{env_name}" 
      ENV_KEY="{env_key}"
      ENV_VALUE="{env_value}" 


      # Export using indirect parameter expansion
        export $ENV_KEY="$ENV_VALUE"

      # Check if the environment variable is already setup or not.
 
         if [ -z "$(eval echo \$$ENV_KEY)" ]; then
            echo -e "$BLACK_BACKGROUND$BOLD_RED_FOREGROUND ERROR -> Environment Variable '$ENV_NAME' is not set !!!!!! $RESET";
         else
            echo "___________________________________________";
            echo "___________________________________________";
            echo -e "$BLACK_BACKGROUND$BOLD_WHITE_FOREGROUND Environment variable '$ENV_NAME' is applied... $RESET"
            echo -e "$BLACK_BACKGROUND$BOLD_RED_FOREGROUND Note : Environment Variable will be Available in this \n Shell or Terminal or Tab Only \n ,, it wont work on Other Shells , Terminal Tabs & Terminal Windows.";
            echo -e "if you want to access Environment Variable in Other \n Terminal Tabs , Terminal Windows or Shell \n then Redo this Process in the desired shell/terminal tab. $RESET";

        fi



        #Deleting Script because it contains sensetive information.
        
        SCRIPT_FILE_PATH=$BASH_SOURCE
        #echo "FILE_PATH : $SCRIPT_FILE_PATH"


        rm $SCRIPT_FILE_PATH

        # Check if the script file was successfully deleted
        if [ ! -f "$SCRIPT_FILE_PATH" ]; then
            echo -e "$BLACK_BACKGROUND$BOLD_RED_FOREGROUND Everything Securely Cleaned up............ $RESET"
          else
            echo -e "$BLACK_BACKGROUND$BOLD_RED_FOREGROUND Failed to Cleanup !!!!!!!!!!!!!!!!!!!!!!!!!!!! $RESET"
        fi     
      "#
    );

    // Creating folder "/tmp/batman"

    let directory_path: PathBuf = PathBuf::from("/tmp/batman");
    fs::create_dir_all(&directory_path).expect("Unable to create temporary directory");

    // creating a file and saving script.

    let file_path: PathBuf = directory_path.join(env_name);

    let new_file = File::create(&file_path);
    //.expect("Failed to Create Shell file..");

    match new_file {
        Ok(new_file) => {
            writeln!(&new_file, "{}", shell_script)
                .expect("Unable to Write shell script to the shell  file");
            let string_path: &str = file_path
                .to_str()
                .expect("Failed to Convert PathBuffer to String Slice.");

            println!(
                "Copy and paste this command in your Desired shell/terminal '{}'",
                format!("source {}", string_path).bright_black().on_white()
            );
        }
        Err(error) => {
            println!(
                "ERROR -> {}",
                error.to_string().bright_red().on_bright_black()
            );
        }
    }
}
