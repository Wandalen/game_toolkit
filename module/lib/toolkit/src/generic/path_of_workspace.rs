//!
//! Helpers to find path of worksapce and change current directory to the path.
//!

use std::{ env, path::PathBuf, process };

/// Namespace to include with asterisk.
pub mod prelude
{
}

/// Get path of worksapce or return current if fail to get path of workspace.
pub fn path_of_workspace() -> PathBuf
{
  let current_dir = env::current_dir().unwrap();
  let cmd_output = process::Command::new( "cargo" )
  .args([ "metadata", "--format-version=1" ])
  .output();

  if let Ok( output ) = cmd_output
  {
    if !output.status.success()
    {
      return current_dir;
    }

    let output2 = String::from_utf8( output.stdout );
    if let Err( _ ) = output2
    {
      return current_dir;
    }

    let json = serde_json::from_str::< serde_json::Value >
    ( output2.unwrap().as_str() );
    if let Ok( json ) = json
    {
      let path = match json.get( "workspace_root" )
      {
        Some( val ) => match val.as_str()
        {
          Some( val ) => val,
          None => return current_dir,
        },
        None => return current_dir,
      };
      PathBuf::from( path )
    }
    else
    {
      return current_dir;
    }
  }
  else
  {
    return current_dir;
  }

}

///
/// Set workspace path as current directory.
///
/// ### Sample
///
///  use game_tookit::generic::path_of_workspace::path_of_workspace_cd;
///  path_of_workspace_cd();
///
pub fn path_of_workspace_cd() -> Option< () >
{
  let path_of_workspace = path_of_workspace();
  env::set_current_dir( path_of_workspace ).ok()?;
  Some( () )
}
