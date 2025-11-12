#![cfg(target_os = "linux")]
use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::sched::{CloneFlags, unshare};
use nix::sys::resource::{Resource, setrlimit};
use std::path::{Path, PathBuf};

/// 沙盒配置结构
pub struct SandboxConfig<'a> {
    pub tmp_root: &'a str,
    pub mount_proc: bool,
    pub mount_tmp: bool,
    pub readonly_root: bool,
    pub bind_dirs: Vec<(&'a str, &'a str, bool)>, // (host_path, sandbox_path, readonly)

    pub cpu_limit_secs: u64,
    pub mem_limit_bytes: u64,
    pub nofile_limit: u64,
    pub nproc_limit: u64,

    pub enable_pid_ns: bool,
    pub enable_user_ns: bool,
    pub enable_mount_ns: bool,
    pub enable_net_ns: bool,
    pub enable_uts_ns: bool,
}

impl<'a> Default for SandboxConfig<'a> {
    fn default() -> Self {
        SandboxConfig {
            tmp_root: "/tmp/sandbox",
            mount_proc: true,
            mount_tmp: true,
            readonly_root: true,
            bind_dirs: vec![("/usr/bin", "/tmp/sandbox/bin", true)],

            cpu_limit_secs: 5,
            mem_limit_bytes: 256 * 1024 * 1024,
            nofile_limit: 64,
            nproc_limit: 32,

            enable_pid_ns: true,
            enable_user_ns: true,
            enable_mount_ns: true,
            enable_net_ns: false,
            enable_uts_ns: false,
        }
    }
}

/// 沙盒本体
pub struct Sandbox<'a> {
    config: SandboxConfig<'a>,
}

impl<'a> Sandbox<'a> {
    pub fn new(config: SandboxConfig<'a>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut flags = CloneFlags::empty();
        if config.enable_mount_ns {
            flags |= CloneFlags::CLONE_NEWNS;
        }
        if config.enable_pid_ns {
            flags |= CloneFlags::CLONE_NEWPID;
        }
        if config.enable_user_ns {
            flags |= CloneFlags::CLONE_NEWUSER;
        }
        if !flags.is_empty() {
            unshare(flags)?;
        }

        // 根挂载
        let mut ms_flags = MsFlags::empty();
        if config.readonly_root {
            ms_flags |= MsFlags::MS_RDONLY;
        }
        mount(
            Some("tmpfs"),
            config.tmp_root,
            Some("tmpfs"),
            ms_flags,
            None::<&str>,
        )?;

        // /proc
        if config.mount_proc {
            let proc_path = PathBuf::from(format!("{}/proc", config.tmp_root));
            std::fs::create_dir_all(&proc_path)?;
            mount(
                Some("proc"),
                &proc_path,
                Some("proc"),
                MsFlags::empty(),
                None::<&str>,
            )?;
        }

        // /tmp
        if config.mount_tmp {
            let tmp_path = PathBuf::from(format!("{}/tmp", config.tmp_root));
            std::fs::create_dir_all(&tmp_path)?;
            mount(
                Some("tmpfs"),
                &tmp_path,
                Some("tmpfs"),
                MsFlags::empty(),
                None::<&str>,
            )?;
        }

        // bind dirs
        for (host, target, readonly) in &config.bind_dirs {
            std::fs::create_dir_all(target)?;
            let mut bind_flags = MsFlags::MS_BIND;
            if *readonly {
                bind_flags |= MsFlags::MS_RDONLY;
            }
            mount(
                Some(&PathBuf::from(host)),
                &PathBuf::from(target),
                None::<&str>,
                bind_flags,
                None::<&str>,
            )?;
        }

        // 资源限制：直接用 u64
        setrlimit(
            Resource::RLIMIT_CPU,
            config.cpu_limit_secs,
            config.cpu_limit_secs,
        )?;
        setrlimit(
            Resource::RLIMIT_AS,
            config.mem_limit_bytes,
            config.mem_limit_bytes,
        )?;
        setrlimit(
            Resource::RLIMIT_NOFILE,
            config.nofile_limit,
            config.nofile_limit,
        )?;
        setrlimit(
            Resource::RLIMIT_NPROC,
            config.nproc_limit,
            config.nproc_limit,
        )?;

        Ok(Self { config })
    }

    pub fn release(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut paths = vec![self.config.tmp_root.to_string()];
        if self.config.mount_proc {
            paths.push(format!("{}/proc", self.config.tmp_root));
        }
        if self.config.mount_tmp {
            paths.push(format!("{}/tmp", self.config.tmp_root));
        }

        for path in paths {
            if Path::new(&path).exists() {
                umount2(&PathBuf::from(&path), MntFlags::MNT_DETACH)?;
            }
        }
        Ok(())
    }

    pub fn run<F, R>(config: SandboxConfig<'a>, f: F) -> Result<R, Box<dyn std::error::Error>>
    where
        F: FnOnce() -> R,
    {
        let sandbox = Self::new(config)?;
        let result = f();
        sandbox.release()?;
        Ok(result)
    }
}
