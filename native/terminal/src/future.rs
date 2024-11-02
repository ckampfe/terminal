fn load(env: Env, _: Term) -> bool {
    env.register::<FutureResource>().unwrap();
    true
}

struct FutureResource((Mutex<Option<(OwnedEnv, SavedTerm)>>, Condvar));

impl Resource for FutureResource {}

#[rustler::nif(schedule = "DirtyIo")]
fn lock_terminal_with_callback<'env>(
    env: Env<'env>,
    terminal: ResourceArc<TerminalResource>,
    callback: Term,
    args: Vec<Term>,
) -> Term<'env> {
    // let mut lock = terminal.terminal.lock().unwrap();

    // let ltr = ResourceArc::new(LockedTerminalRef(lock));

    // let mut nargs = args.clone();
    // let mut nargs = vec![];
    let tt = ResourceArc::clone(&terminal).encode(env);

    // nargs.push(ResourceArc::clone(&terminal).encode(env));

    // for arg in &args {
    //     nargs.push(*arg)
    // }

    let fut = run_elixir_callback(env, callback, args);

    loop {
        let (lock, cvar) = &fut.0;
        let guard = lock.lock().unwrap();

        let result = cvar
            .wait_timeout(guard, std::time::Duration::from_secs(5))
            .unwrap();

        if let Some((owned_env, t)) = result.0.as_ref() {
            let out = owned_env.run(|the_owned_env| {
                let term = t.load(the_owned_env);
                term.in_env(env)
            });
            break out;
        }
    }
}

fn run_elixir_callback(env: Env, f: rustler::Term, args: Vec<Term>) -> ResourceArc<FutureResource> {
    let fut = ResourceArc::new(FutureResource((
        Mutex::new(None),
        std::sync::Condvar::new(),
    )));

    let name = rustler::Atom::from_str(env, "Elixir.Terminal.CallbackServer").unwrap();

    let callback_server = env.whereis_pid(name).unwrap();

    env.send(
        &callback_server,
        (
            rustler::Atom::from_str(env, "execute_callback").unwrap(),
            (f, args, ResourceArc::clone(&fut)),
        ),
    )
    .unwrap();

    fut
}

#[rustler::nif]
fn complete_future(future_ref: ResourceArc<FutureResource>, result: Term) {
    let (lock, cvar) = &future_ref.0;
    let mut started = lock.lock().unwrap();

    let owned_env = rustler::OwnedEnv::new();
    let saved_result = owned_env.save(result);

    *started = Some((owned_env, saved_result));

    cvar.notify_one();
}
